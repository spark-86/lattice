use std::fs;

use anyhow::Result;
use iam::IAm;
use lattice::{Rhex, rhex::intent::RhexIntent, scope::Scope};
use transform::{descriptor::DescriptorAction, registry::TransformRegistry};

use crate::{
    check::{self, CheckStatus},
    config::UsherdConfig,
    firing,
};

pub fn append(
    config: &UsherdConfig,
    scope: &mut Scope,
    rhex: &Rhex,
    trans_registry: TransformRegistry,
    iam: &IAm,
) -> Result<(Vec<CheckStatus>, Option<Vec<RhexIntent>>)> {
    let mut outputs = Vec::new();
    outputs.push(check::check_data_size(rhex)?);
    outputs.push(check::check_schema(rhex)?);
    outputs.push(check::check_same_scope(scope, rhex)?);
    outputs.push(check::check_prev(scope, rhex)?);
    outputs.push(check::check_nonce(scope, rhex)?);
    outputs.push(check::check_rt_access(scope, rhex)?);
    outputs.push(check::check_usher(scope, rhex)?);
    // Check all the sigs
    for i in 0..rhex.sigs.len() {
        outputs.push(check::check_sig(rhex, i)?);
    }
    // Make sure we are the usher being submitted to
    if !iam.am_i(&rhex.intent.usher)? {
        outputs.push(CheckStatus::InvalidUsher);
    }

    // Fire transforms
    let (status, intents) =
        firing::fire_transforms(rhex, trans_registry, DescriptorAction::Validate)?;
    outputs.push(status);

    // Strip all the successes and see if there's anything left
    outputs.retain(|s| *s != CheckStatus::Success);
    if outputs.len() > 0 {
        return Ok((outputs, None));
    };

    // Ok, we have the all clear.

    // Do the physical append
    let rhex_file = fs::read(format!("{}{}/records.rhex", &config.scopes, &scope.name))?;
    let mut rhex_objs: Vec<Rhex> = minicbor::decode(&rhex_file)?;
    rhex_objs.push(rhex.clone());
    let mut rhex_file = Vec::new();
    minicbor::encode(rhex_objs, &mut rhex_file)?;
    fs::write(
        format!("{}{}/records.rhex", &config.scopes, &scope.name),
        &rhex_file,
    )?;
    // Update the scope head
    scope.head = rhex.curr.clone();

    Ok((vec![CheckStatus::Success], Some(intents)))
}

pub enum AppendError {}
