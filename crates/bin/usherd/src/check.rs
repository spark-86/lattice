use anyhow::{Ok, Result};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use lattice::{Rhex, scope::Scope};

/// # check_curr_hash
/// Checks if the Rhex's proposed `curr` matches the calculated
/// final hash of the object
///
pub fn check_curr_hash(rhex: &Rhex) -> Result<CheckStatus> {
    if rhex.curr.is_none() || (rhex.calc_curr() != rhex.curr.unwrap()) {
        return Ok(CheckStatus::CurrentHashMismatch {
            presented: rhex.curr,
            calculated: rhex.calc_curr(),
        });
    };
    Ok(CheckStatus::Success)
}

/// # check_same_scope
/// Checks to make sure this Rhex is for this Scope.
///
pub fn check_same_scope(scope: &Scope, rhex: &Rhex) -> Result<CheckStatus> {
    // See if this record is even for this scope
    if scope.name != rhex.intent.scope {
        return Ok(CheckStatus::NotThisScope {
            presented: rhex.intent.scope.to_string(),
            expected: scope.name.to_string(),
        });
    };
    Ok(CheckStatus::Success)
}

/// # check_prev
/// Makes sure the presented `intent.prev` matches the scope head,
/// or `curr` of the last Rhex in the Scope
pub fn check_prev(scope: &Scope, rhex: &Rhex) -> Result<CheckStatus> {
    // Does previous match head?
    if rhex.intent.prev != scope.head {
        return Ok(CheckStatus::PrevHashMismatch {
            presented: rhex.intent.prev,
            expected: scope.head,
        });
    };
    Ok(CheckStatus::Success)
}

/// # check_nonce
/// Checks to see if the `intent.nonce` has been reused for this
/// Scope
///
pub fn check_nonce(scope: &Scope, rhex: &Rhex) -> Result<CheckStatus> {
    // skim for nonce reuse
    if scope.check_nonce_reused(&rhex.intent.nonce) {
        return Ok(CheckStatus::NonceReused);
    };
    Ok(CheckStatus::Success)
}

/// # check_rt_access
/// check to make sure we can append this record as the author
/// at this coordinate.
///    
pub fn check_rt_access(scope: &Scope, rhex: &Rhex) -> Result<CheckStatus> {
    let groups = scope.member_of_at(rhex.intent.author.clone(), rhex.context.at.clone())?;
    if groups.len() == 0 {
        return Ok(CheckStatus::AccessDenied);
    }
    let policy = scope.get_policy_at(rhex.context.at.clone());
    let submittable = policy.can_submit(rhex.intent.rt, &groups);
    if !submittable {
        return Ok(CheckStatus::RtNotAllowed);
    }
    Ok(CheckStatus::Success)
}

/// # check_usher
/// Currently just checks to see if the Usher is listed as valid
/// in the Scope. Future this may actually do more.
///
pub fn check_usher(scope: &Scope, rhex: &Rhex) -> Result<CheckStatus> {
    // see if usher specified is available.
    let mut current_usher = scope.ushers_at(rhex.context.at.clone())?;
    current_usher.retain(|u| u.0 == rhex.intent.usher);
    if current_usher.len() == 0 {
        return Ok(CheckStatus::InvalidUsher);
    }
    Ok(CheckStatus::Success)
}

/// # check_data_size
/// Very basically checks to see if the CBOR data size is over 1k
///
pub fn check_data_size(rhex: &Rhex) -> Result<CheckStatus> {
    if rhex.data_size() > 1024 {
        return Ok(CheckStatus::DataBloated);
    }
    Ok(CheckStatus::Success)
}

/// # check_schema
/// This is supposed to check the `intent.schema` and validate
/// against it. It currently does none of this lol.
///
pub fn check_schema(_rhex: &Rhex) -> Result<CheckStatus> {
    // TODO: actually do this? lol
    Ok(CheckStatus::Success)
}

/// # check_time_reversal
/// Basically just makes sure we're not trying to creep in an
/// 'earlier' Rhex
///
pub fn check_time_reversal(scope: &Scope, rhex: &Rhex) -> Result<CheckStatus> {
    // Have we gone backwards in time?
    let latest = scope.latest_time();
    if latest > rhex.context.at {
        return Ok(CheckStatus::TimeReversal {
            presented: rhex.context.at.clone(),
            prev: latest,
        });
    }
    Ok(CheckStatus::Success)
}

pub fn check_sig(rhex: &Rhex, pos: usize) -> Result<CheckStatus> {
    let key = VerifyingKey::from_bytes(&rhex.sigs[pos].pk.clone())?;
    let status = key.verify(
        &rhex.get_hash(rhex.sigs[pos].t.clone()),
        &Signature::from_bytes(&rhex.sigs[pos].sig),
    );
    if status.is_err() {
        return Ok(CheckStatus::SignatureInvalid(pos.try_into().unwrap()));
    }
    Ok(CheckStatus::Success)
}

/// # CheckStatus
/// This is all the possible outcomes of the "check" functions
///
#[derive(Debug, Clone, PartialEq)]
pub enum CheckStatus {
    Success,
    PrevHashMismatch {
        presented: Option<[u8; 32]>,
        expected: Option<[u8; 32]>,
    },
    NotThisScope {
        presented: String,
        expected: String,
    },
    NonceReused,
    AccessDenied,
    InvalidUsher,
    RtNotAllowed,
    DataBloated,
    InteractionAborted {
        transform: String,
        exitcode: usize,
    },
    SchemaFailed(String),
    SchemaNotFound(String),
    TimeReversal {
        presented: u64,
        prev: u64,
    },
    SpacialDataIncorrect(String),
    SignatureInvalid(u8),
    CurrentHashMismatch {
        presented: Option<[u8; 32]>,
        calculated: [u8; 32],
    },
    RhexBloated(usize),
    NotUsherForThisScope,
}
