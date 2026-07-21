use anyhow::Result;
use minicbor::{Decode, Encode};
use rhex::intent::RhexIntent;

use crate::error::TransformError;

/// #TransformOutput
/// This should be what is deserialized from the
/// output [u8], with outbound intents getting routed
/// out.
#[derive(Debug, Clone, Encode, Decode)]
pub struct TransformOutput {
    #[b(0)]
    pub outbound_intents: Option<Vec<RhexIntent>>,
    #[n(1)]
    pub err: Option<Vec<TransformError>>,
}

impl TransformOutput {
    pub fn new() -> Self {
        Self {
            outbound_intents: None,
            err: None,
        }
    }

    /// # fatal_error
    /// returns true if the transform should cause the current
    /// action to fail and stop processing the Rhex.
    ///
    pub fn fatal_error(&self) -> bool {
        if self.err.is_some() {
            for err in self.err.as_ref().unwrap() {
                if err.severity == crate::error::TransformErrorSeverity::Fatal
                    || err.severity == crate::error::TransformErrorSeverity::Error
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn from_slice(data: &[u8]) -> Result<Self> {
        Ok(minicbor::decode(data).unwrap())
    }
}
