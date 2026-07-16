/* use std::{
    collections::{HashMap, hash_map::Entry},
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::Result;
use lattice::{
    Lattice, Rhex,
    rhex::data::RhexData,
    scope::{Scope, policy::Policy},
};
use serde::{Deserialize, Serialize};
use time::MicroMarks;
use transform::{
    context::TransformContext, descriptor::DescriptorAction, output::TransformOutput,
    registry::TransformRegistry,
};

pub fn walk(
    rhex: Vec<Rhex>,
    registry: &mut TransformRegistry,
) -> Result<(ScopeWalkResult, Option<Scope>)> {
    // If there's no R⬢, there can be no scope.
    if rhex.len() == 0 {
        return Ok((ScopeWalkResult::ZeroLength, None));
    }
    // Get the scope name from the first R⬢
    let scope = rhex[0].intent.scope.clone();
    let mut working_scope = Scope::new(scope.clone(), Lattice::GENESIS_KEY);
    let mut pos = 0;

    for r in &rhex {
        // First off the rip we make sure the Rhex validates itself.
        if !r.validate() {
            return Ok((
                ScopeWalkResult::FailedValidation {
                    chain_pos: pos,
                    hash: r.curr,
                    reason: "Failed internal audit".to_string(),
                },
                None,
            ));
        }

        // We check to make sure the current policy allows for this
        // submission.
        //
        // First we collect all the groups our author could be in
        let user_groups =
            working_scope.member_of_at(r.intent.author.clone(), r.context.at.clone())?;
        if user_groups.len() == 0 {
            return Ok((
                ScopeWalkResult::PolicyViolation {
                    chain_pos: pos,
                    violation: PolicyViolation::InvalidKey,
                },
                None,
            ));
        }
        // Grab the policy at this micromark
        let working_policy = working_scope.get_policy_at(r.context.at.clone());
        // Can we append with this policy?
        let result = working_policy.can_submit(&r.intent.rt, &user_groups);
        if !result {
            return Ok((
                ScopeWalkResult::PolicyViolation {
                    chain_pos: pos,
                    violation: PolicyViolation::InvalidKey,
                },
                None,
            ));
        }

        // Now we append
        // First we check for policy/key records, because we should be
        // the one that processes them first.
        match r.intent.rt.as_str() {
            "policy:set" => {
                let policy_bin = match &r.intent.data {
                    RhexData::Binary { data } => data,
                    _ => anyhow::bail!("Invalid data type for policy!"),
                };
                let policy: Policy = serde_cbor::from_slice(&policy_bin).unwrap();
                working_scope
                    .policy_map
                    .insert((policy.eff, policy.exp), policy);
            }
            _ => {}
        }

        // What this requires is walking the triggers hashmap for this
        // scope we are looking at. We fire each one per hash.
        let action_set = registry
            .triggers
            .entry(DescriptorAction::Validate)
            .or_insert(HashMap::new());
        let scope_set = action_set.entry(scope.clone()).or_insert(HashMap::new());
        let rt = scope_set.entry(r.intent.rt.clone()).or_insert(Vec::new());
        'outer: for entry in rt {
            match registry.registry.entry(*entry) {
                Entry::Occupied(occ) => {
                    let transform = occ.get();
                    // TODO: Add input gathering
                    let mut ctx = TransformContext {
                        input: &r.to_vec(),
                        output: &mut None,
                        diag: &mut None,
                    };
                    println!("Firing transform {}...", transform.descriptor.name);
                    // FIRE!
                    let result = (transform.entry.entry)(&mut ctx);
                    if result > 0 {
                        // aw shit... :(
                        let output: TransformOutput =
                            serde_cbor::from_slice(&ctx.output.clone().unwrap()).unwrap();
                        println!(
                            "Error occured with transform {}, returned {}",
                            transform.descriptor.name, result
                        );
                        if output.err.is_some() {
                            let errors = output.err.clone().unwrap();
                            for err in errors {
                                println!("{:?}", err);
                            }
                        }
                        // if any of the errors are considered fatal, we
                        // halt any further transform action. This is
                        // to prevent a run away or for it to keep churning
                        // garbage data.
                        if output.fatal_error() {
                            println!("💀 FATAL ERROR! Execution halted.");
                            break 'outer;
                        }
                    }
                }
                _ => {}
            }
        }
        // Actually attach to the working scope
        working_scope.add_rhex(r.clone());
        working_scope.head = r.curr;
        let time = SystemTime::now();
        let time = time
            .duration_since(UNIX_EPOCH)
            .expect("Whoops... time error")
            .as_millis();
        working_scope.updated = (time as i64).as_micromarks();

        // Advance the position
        pos += 1;
    }
    Ok((ScopeWalkResult::Success(pos), Some(working_scope)))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScopeWalkResult {
    // Success, with the record count we walked
    Success(u64),
    // Failed basic R⬢ validation
    FailedValidation {
        chain_pos: u64,
        hash: Option<[u8; 32]>,
        reason: String,
    },
    // The previous R⬢ does not line up with the current
    LinkMismatch {
        prev_rhex: [u8; 32],
        intent_prev: Option<[u8; 32]>,
        curr: [u8; 32],
    },
    // A R⬢ was submitted outside the policy
    PolicyViolation {
        chain_pos: u64,
        violation: PolicyViolation,
    },
    // In case there's no R⬢ in the scope
    ZeroLength,
    // ???
    Unknown,
}

/// # PolicyViolation
/// I tried to include all the failure states for a policy I
/// could think of, but I'm sure I'm missing something.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyViolation {
    InvalidKey,
    RtNotAllowed,
    RateExceeded,
    OutsideWindow,
}
    */
