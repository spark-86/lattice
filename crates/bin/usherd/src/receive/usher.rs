use anyhow::Result;
use key::enclave::Enclave;
use lattice::{
    Rhex,
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
    firing,
};

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
    //let mut storage = Vec::new();
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
