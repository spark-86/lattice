use anyhow::Result;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};

use crate::Rhex;

impl Rhex {
    /// # check_curr_hash
    /// Checks if the Rhex's proposed `curr` matches the calculated
    /// final hash of the object
    ///
    pub fn check_curr_hash(&self) -> Result<CheckStatus> {
        let calc_curr = self.calc_curr();
        if self.curr.is_none() || (calc_curr != self.curr.unwrap()) {
            return Ok(CheckStatus::CurrentHashMismatch {
                presented: self.curr,
                calculated: calc_curr,
            });
        };
        Ok(CheckStatus::Success)
    }

    /// # check_data_size
    /// Very basically checks to see if the CBOR data size is over 1k
    ///
    pub fn check_data_size(&self) -> Result<CheckStatus> {
        let size = self.data_size();
        if size > 1024 {
            return Ok(CheckStatus::DataBloated(size));
        }
        Ok(CheckStatus::Success)
    }

    /// # check_schema
    /// This is supposed to check the `intent.schema` and validate
    /// against it. It currently does none of this lol.
    ///
    pub fn check_schema(&self) -> Result<CheckStatus> {
        // TODO: actually do this? lol
        Ok(CheckStatus::Success)
    }

    /// # check_sig
    /// Checks a singular signature by position in rhex.sigs
    ///
    pub fn check_sig(&self, pos: usize) -> Result<CheckStatus> {
        let key = VerifyingKey::from_bytes(&self.sigs[pos].pk.clone())?;
        let status = key.verify(
            &self.get_hash(self.sigs[pos].t.clone()),
            &Signature::from_bytes(&self.sigs[pos].sig),
        );
        if status.is_err() {
            return Ok(CheckStatus::SignatureInvalid(pos.try_into().unwrap()));
        }
        Ok(CheckStatus::Success)
    }

    /// # check_total_size
    /// Checks the completed total size of the Rhex and errors if it's
    /// over 4k
    ///
    pub fn check_total_size(&self) -> Result<CheckStatus> {
        let mut buf = Vec::new();
        minicbor::encode(self, &mut buf)?;
        if buf.len() > 4096 {
            return Ok(CheckStatus::RhexBloated(buf.len()));
        }
        Ok(CheckStatus::Success)
    }
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
    DataBloated(usize),
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
    CurrentHashNotSet,
    RhexBloated(usize),
    NotUsherForThisScope,
}
