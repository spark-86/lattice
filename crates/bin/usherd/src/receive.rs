use anyhow::Result;
use key::enclave::Enclave;
use lattice::{
    Lattice, Rhex,
    rhex::{
        intent::RhexIntent,
        signature::{RhexSignature, RhexSignatureType},
    },
    scope::Scope,
};
use time::MicroMarks;
use transform::{descriptor::DescriptorAction, registry::TransformRegistry};

use crate::{
    check::{self, CheckStatus},
    config::UsherdConfig,
    firing,
    iam::{self, IAm},
};

pub fn receive(
    config: &UsherdConfig,
    rhex: Rhex,
    trans_registry: TransformRegistry,
    lattice: Lattice,
) -> Result<(ReceiveStatus, Vec<Rhex>)> {
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
            let mut enclave = Enclave::new(Some(config.enclave.clone()));
            let _ = enclave.populate();
            let scope = lattice.scopes.get(&rhex.intent.scope).unwrap();
            let (status, signed, intents) = recv_usher_sig(scope, &rhex, trans_registry, &enclave)?;

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
            let scope = lattice.scopes.get(&rhex.intent.scope).unwrap();
            let policy = scope.get_policy_at(rhex.context.at.clone());
            let groups = scope.member_of_at(rhex.intent.author, rhex.context.at)?;
            let k = policy.get_k(&rhex.intent.rt, &groups)?;
            if k > 0 {}
        }
        3.. => {
            // We have quorum assembled and are submitting for final
            // recording
        }
    };
    Ok((ReceiveStatus::Success, vec![]))
}

pub fn recv_usher_sig(
    scope: &Scope,
    rhex: &Rhex,
    trans_registry: TransformRegistry,
    enclave: &Enclave,
) -> Result<(CheckStatus, Option<Rhex>, Vec<RhexIntent>)> {
    let mut outputs = Vec::new();
    // check all the pertanent things
    outputs.push(check::check_data_size(rhex)?);
    outputs.push(check::check_schema(rhex)?);
    outputs.push(check::check_same_scope(scope, rhex)?);
    outputs.push(check::check_prev(scope, rhex)?);
    outputs.push(check::check_nonce(scope, rhex)?);
    outputs.push(check::check_rt_access(scope, rhex)?);
    outputs.push(check::check_usher(scope, rhex)?);
    outputs.push(check::check_sig(rhex, 0)?);
    // check for transforms for validation
    let (status, intents) =
        firing::fire_transforms(rhex, trans_registry, DescriptorAction::Validate)?;
    outputs.push(status);
    outputs.retain(|s| *s != CheckStatus::Success);
    if outputs.len() > 0 {
        return Ok((outputs[0].clone(), None, vec![]));
    };
    // sign
    let mut new_rhex = rhex.clone();
    let sig = enclave.sign(&rhex.intent.usher, &rhex.get_hash(RhexSignatureType::Usher))?;
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    // set time
    new_rhex.context.at = time.as_micromarks();
    // TODO: Add spacial data here... I haven't decided where the
    // usher stores that at this moment so we're just gonna leave this
    // here for now.
    new_rhex.context.s = None;
    new_rhex.sigs.push(RhexSignature {
        pk: rhex.intent.usher.clone(),
        sig,
        t: RhexSignatureType::Usher,
    });
    // return
    Ok((CheckStatus::Success, Some(new_rhex), intents))
}

pub fn recv_quorum_sig(
    scope: &Scope,
    rhex: &Rhex,
    enclave: &Enclave,
    me: &IAm,
) -> Result<(CheckStatus, Option<RhexSignature>)> {
    if check::check_sig(rhex, 0)? == CheckStatus::Success
        && check::check_sig(rhex, 1)? == CheckStatus::Success
    {
        let policy = scope.get_policy_at(rhex.context.at);
        let quorum_key = iam::is_usher(me, scope, rhex.context.at)?;
        let quorum_key = if quorum_key.is_some() {
            quorum_key.unwrap()
        } else {
            return Ok((CheckStatus::NotUsherForThisScope, None));
        };
        let window = policy.get_window(
            &rhex.intent.rt,
            &scope.member_of_at(rhex.intent.author, rhex.context.at)?,
        )?;
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis() as i64;
        if (time.as_micromarks() - window) < rhex.context.at {
            // Our sig request was received in the appropriate window
            let sig = enclave.sign(&quorum_key, &rhex.get_hash(RhexSignatureType::Quorum))?;
            let sig_rec = {
                RhexSignature {
                    pk: quorum_key.clone(),
                    sig,
                    t: RhexSignatureType::Quorum,
                }
            };
            return Ok((CheckStatus::Success, Some(sig_rec)));
        };
    }
    Ok((CheckStatus::SignatureInvalid(0), None))
}

pub enum ReceiveStatus {
    Success,
    FailedValidation(CheckStatus),
    MissingSignature(String),
}
