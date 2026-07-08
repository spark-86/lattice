use anyhow::Result;
use lattice::{Rhex, rhex::intent::RhexIntent};
use transform::{
    context::TransformContext, descriptor::DescriptorAction, output::TransformOutput,
    registry::TransformRegistry,
};

use crate::check::CheckStatus;

/// # fire_transforms
/// This has to be the worst possible way to do this. This is the damn
/// CRE creeping into this. I can't help it. I just feel like the
/// modular execution is important to the Lattice.
pub fn fire_transforms(
    rhex: &Rhex,
    trans_registry: TransformRegistry,
    action: DescriptorAction,
) -> Result<(CheckStatus, Vec<RhexIntent>)> {
    let mut output_intents = Vec::new();
    let action_set = trans_registry.triggers.get(&action);
    if action_set.is_some() {
        let triggers = action_set.unwrap().get(&rhex.intent.scope);
        if triggers.is_some() {
            let triggers = triggers.unwrap();
            let keys: Vec<&String> = triggers.keys().collect();
            let parent = &format!("{}:*", rhex.intent.rt.split(":").collect::<Vec<&str>>()[0]);
            for key in keys {
                if key == parent || *key == rhex.intent.rt {
                    for trans in triggers.get(key).unwrap() {
                        let entry = trans_registry.registry.get(trans);
                        if entry.is_some() {
                            let entry = entry.unwrap();
                            // TODO: build additional inputs here
                            let mut ctx = TransformContext {
                                input: &serde_cbor::to_vec(&rhex).unwrap(),
                                output: &mut None,
                                diag: &mut None,
                            };
                            let result = (entry.entry.entry)(&mut ctx);
                            if result > 0 {
                                let errs = ctx.output.clone().unwrap();
                                let errs: TransformOutput = serde_cbor::from_slice(&errs).unwrap();
                                if errs.fatal_error() {
                                    return Ok((
                                        CheckStatus::InteractionAborted {
                                            transform: entry.descriptor.name.clone(),
                                            exitcode: result as usize,
                                        },
                                        vec![],
                                    ));
                                }
                            } else {
                                let output = ctx.output;
                                if output.is_some() {
                                    let oi: TransformOutput =
                                        serde_cbor::from_slice(&output.clone().unwrap()).unwrap();
                                    let intents = oi.outbound_intents;
                                    if intents.is_some() {
                                        for intent in intents.unwrap() {
                                            output_intents.push(intent.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok((CheckStatus::Success, output_intents))
}
