use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhexContext {
    pub at: u64,
    pub s: Option<ContextSpacial>,
}

impl RhexContext {
    pub fn new(at: u64, s: Option<ContextSpacial>) -> Self {
        Self { at, s }
    }

    pub fn get_hash(&self) -> [u8; 32] {
        blake3::hash(&serde_cbor::to_vec(&self).unwrap()).into()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSpacial {
    pub s_ref: String,
    pub s_data: Vec<u8>,
}

impl ContextSpacial {
    pub fn new(s_ref: String, s_data: Vec<u8>) -> Self {
        Self { s_ref, s_data }
    }
}
