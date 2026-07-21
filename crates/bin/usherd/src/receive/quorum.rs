use anyhow::Result;
use iam::IAm;
use key::enclave::Enclave;
use lattice::{
    Rhex,
    rhex::signature::{RhexSignature, RhexSignatureType},
    scope::{Scope, ushers::UsherRole},
};
use time::MicroMarks;

use crate::check::{self, CheckStatus};

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
        let quorum_key = me.pick(scope, rhex.context.at, UsherRole::Quorum)?;
        let quorum_key = if quorum_key.is_some() {
            quorum_key.unwrap()
        } else {
            return Ok((CheckStatus::NotUsherForThisScope, None));
        };
        let window = policy.get_window(
            &rhex.intent.rt.to_string(),
            &scope.member_of_at(rhex.intent.author, rhex.context.at)?,
        )?;
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis() as i64;
        if (time.as_micromarks() - window) < rhex.context.at {
            // Our sig request was received in the appropriate window
            let sig = enclave.sign(
                &quorum_key,
                &rhex.get_hash(RhexSignatureType::Quorum(time.as_micromarks())),
            )?;
            let sig_rec = {
                RhexSignature {
                    pk: quorum_key.clone(),
                    sig,
                    t: RhexSignatureType::Quorum(time.as_micromarks()),
                }
            };
            return Ok((CheckStatus::Success, Some(sig_rec)));
        };
    }
    Ok((CheckStatus::SignatureInvalid(0), None))
}
