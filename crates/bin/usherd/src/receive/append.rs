use std::path::PathBuf;

use anyhow::Result;
use iam::IAm;
use lattice::{
    Rhex,
    rhex::{check::CheckStatus, intent::RhexIntent},
    scope::Scope,
};
use transform::{descriptor::DescriptorAction, registry::TransformRegistry};

use crate::{config::UsherdConfig, firing};

pub fn append(
    config: &UsherdConfig,
    scope: &mut Scope,
    rhex: &Rhex,
    trans_registry: TransformRegistry,
    iam: &IAm,
) -> Result<(Vec<CheckStatus>, Option<Vec<RhexIntent>>)> {
    // TODO: nonce check
    let mut outputs = Vec::new();
    let mut check = scope.final_check(rhex)?;
    outputs.append(&mut check);
    // Make sure we are the usher being submitted to
    if !iam.am_i(&rhex.intent.usher)? {
        outputs.push(CheckStatus::InvalidUsher);
    }

    // Fire transforms
    let (status, intents) =
        firing::fire_transforms(rhex, trans_registry, DescriptorAction::Appending)?;
    outputs.push(status);

    // Strip all the successes and see if there's anything left
    outputs.retain(|s| *s != CheckStatus::Success);
    if outputs.len() > 0 {
        return Ok((outputs, Some(intents)));
    };

    // Ok, we have the all clear.

    // Do the physical append
    // Filename is "./scopes/scopename.rhex"
    let scope_name = if scope.name == "".to_string() {
        "-root-".to_string()
    } else {
        scope.name.clone()
    };
    let filename = PathBuf::from(format!("{}{}.rchain", &config.scopes, &scope_name));
    let mut rhex_objs = Rhex::chain_from_disk(filename.clone())?;

    rhex_objs.push(rhex.clone());
    Rhex::chain_to_disk(filename, rhex_objs)?;
    // Update the scope head
    scope.head = rhex.curr.clone();
    // TODO: Send out a chain update to our sister ushers.

    Ok((vec![CheckStatus::Success], Some(intents)))
}
