use anyhow::Result;
use iam::IAm;
use key::enclave::Enclave;
use lattice::{
    Lattice, Rhex,
    rhex::{
        data::RhexData,
        signature::{RhexSignature, RhexSignatureType},
    },
};
use transform::registry::TransformRegistry;

use crate::{check::CheckStatus, config::UsherdConfig, receive};

pub mod append;
pub mod quorum;
pub mod usher;

pub fn receive(
    config: &UsherdConfig,
    rhex: Rhex,
    trans_registry: TransformRegistry,
    lattice: Lattice,
    me: &IAm,
) -> Result<(ReceiveStatus, Vec<Rhex>)> {
    let mut enclave = Enclave::new(Some(config.enclave.clone()));
    enclave.populate()?;

    // Process the Rhex to see where we are
    match rhex.sigs.len() {
        0 => {
            return Ok((
                ReceiveStatus::MissingSignature("author".to_string()),
                vec![],
            ));
        }
        1 => {
            // We're here to get an usher sig
            let scope = lattice.scopes.get(&rhex.intent.scope.to_string()).unwrap();
            let (status, signed, intents) =
                receive::usher::recv_usher_sig(scope, &rhex, trans_registry, &enclave)?;

            // We are here to sign over all the outbound Rhex that we generated.
            // TODO: Obviously we should probably not just blindly sign shit
            // going out. I guess some filter? I mean I guess when they install
            // transforms they know what the potential output is but like...
            // I dunno... still feels bad.
            let mut output = Vec::new();
            for i in intents {
                let mut r = Rhex::new();
                r.intent = i;
                let sig = enclave.sign(
                    &rhex.intent.author,
                    &rhex.get_hash(RhexSignatureType::Author),
                );
                if sig.is_ok() {
                    r.sigs.push(RhexSignature {
                        pk: rhex.intent.author.clone(),
                        sig: sig.unwrap(),
                        t: RhexSignatureType::Author,
                    });
                    output.push(r);
                }
            }
            if status != CheckStatus::Success {
                return Ok((ReceiveStatus::FailedValidation(status), output));
            }
            if signed.is_some() {
                output.push(signed.unwrap());
                return Ok((ReceiveStatus::Success, output));
            }
            // TODO: This needs to actually make sure we aren't just
            // straight up submitting with a zero quorum scope. So
            // like we need to check `k` and see if it's one or less.
        }
        2 => {
            // We're looking to either submit w/ no quorum or looking
            // to collect quorum from us
            let scope = lattice.scopes.get(&rhex.intent.scope.to_string()).unwrap();
            let policy = scope.get_policy_at(rhex.context.at.clone());
            let groups = scope.member_of_at(rhex.intent.author, rhex.context.at)?;
            let k = policy.get_k(&rhex.intent.rt.to_string(), &groups)?;
            if k > 0 {
                let result = receive::quorum::recv_quorum_sig(scope, &rhex, &enclave, me)?;
                if result.0 == CheckStatus::Success {
                    // Return the sig via Rhex payload here.
                    let sig = result.1.unwrap().clone();
                    let bin_sig = sig.to_vec()?.clone();
                    let data = RhexData::Binary(bin_sig);
                    let hash_bin = data.to_vec()?;
                    let mut hasher = blake3::Hasher::new();
                    hasher.update(&hash_bin);
                    let data_hash = hasher.finalize();
                    // FIXME: we have to change this obviously.
                    // It should be us snagging from the usher matrix
                    // and pulling out all the valid quorum PKs. Then by
                    // priority I guess? fuck if I know dude.
                    let author = sig.pk.clone();
                    // "Usher" here is just the author that submitted
                    let usher = rhex.intent.author.clone();
                    let mut response_rhex = Rhex::build(
                        None,
                        rhex.intent.scope.clone(),
                        None,
                        author,
                        usher,
                        "response:signature".to_string(),
                        None,
                        Some(data_hash.as_bytes().clone()),
                    );
                    response_rhex.data = hash_bin;
                    let sig = enclave
                        .sign(&author, &response_rhex.get_hash(RhexSignatureType::Author))?;

                    response_rhex.sigs.push(RhexSignature {
                        pk: author,
                        sig,
                        t: RhexSignatureType::Author,
                    });

                    return Ok((ReceiveStatus::Success, vec![response_rhex]));
                }
            }
        }
        3.. => {
            // We have quorum assembled and are submitting for final
            // recording
            let scope = lattice.scopes.get(&rhex.intent.scope).unwrap();
            let policy = scope.get_policy_at(rhex.context.at.clone());
            let groups = scope.member_of_at(rhex.intent.author, rhex.context.at)?;
            let k = policy.get_k(&rhex.intent.rt, &groups)?;
            if usize::from(k) < (rhex.sigs.len() - 2) {
                anyhow::bail!("Not enough signatures");
            }
            // TODO: Check sigs to make sure they're all ushers in quorum.
            // TODO: Check make sure we can still submit against the policy.
            // TODO: Make sure we are an admission point.
        }
    };
    Ok((ReceiveStatus::Success, vec![]))
}

pub enum ReceiveStatus {
    Success,
    FailedValidation(CheckStatus),
    MissingSignature(String),
}
