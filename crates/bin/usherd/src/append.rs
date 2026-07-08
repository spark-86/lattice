use anyhow::Result;
use lattice::{Rhex, rhex::intent::RhexIntent, scope::Scope};
use transform::{descriptor::DescriptorAction, registry::TransformRegistry};

use crate::check::CheckStatus;

/// # append
/// This is the core of it all. It's where the next step in a chain
/// walk or an extention of the lattice comes from.
///
pub fn append(
    _scope: &mut Scope,
    _rhex: Rhex,
    _action: DescriptorAction,
    _trans_registry: TransformRegistry,
) -> Result<(CheckStatus, Vec<RhexIntent>)> {
    let output_intents = Vec::new();
    // First off, let's do some basic sanity checks

    // See if the hash matches what is reported

    // TODO: schema validation goes here

    // see if there's transforms to fire

    Ok((CheckStatus::Success, output_intents))
}
