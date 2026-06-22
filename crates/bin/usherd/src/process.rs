use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use lattice::{Lattice, Rhex, scope::Scope};
use serde::{Deserialize, Serialize};
use time::MicroMarks;

pub fn walk(rhex: Vec<Rhex>) -> Result<(ScopeWalkResult, Option<Scope>)> {
    if rhex.len() == 0 {
        return Ok((ScopeWalkResult::Unknown, None));
    }
    let scope = rhex[0].intent.scope.clone();

    let mut working_scope = Scope::new(scope, Some(Lattice::GENESIS_KEY));
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
        // submission
        let mut user_groups = Vec::new();
        for (name, members) in working_scope.groups.iter() {
            if members.contains(&r.intent.author) {
                user_groups.push(name.clone());
            }
        }
        let result = working_scope.policy.can_submit(&r.intent.rt, &user_groups);
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
        // TODO: Call out to Processor for validation outside our
        // core checks

        // Actually attach to the working scope
        working_scope.add_rhex(r.clone());
        working_scope.head = r.curr.unwrap();
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
    // Failed basic Rhex validation
    FailedValidation {
        chain_pos: u64,
        hash: Option<[u8; 32]>,
        reason: String,
    },
    // The previous record does not line up with the current
    LinkMismatch {
        prev_rhex: [u8; 32],
        intent_prev: Option<[u8; 32]>,
        curr: [u8; 32],
    },
    // A Rhex was submitted outside the policy
    PolicyViolation {
        chain_pos: u64,
        violation: PolicyViolation,
    },
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
