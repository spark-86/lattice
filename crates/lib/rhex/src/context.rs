use minicbor::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode)]
pub struct RhexContext {
    #[n(0)]
    pub at: u64,
    #[n(1)]
    #[cbor(borrow)]
    pub s: Option<ContextSpacial>,
}

impl RhexContext {
    pub fn new(at: u64, s: Option<ContextSpacial>) -> Self {
        Self { at, s }
    }

    pub fn get_hash(&self) -> [u8; 32] {
        blake3::hash(&minicbor::to_vec(&self).unwrap()).into()
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct ContextSpacial {
    #[n(0)]
    pub s_ref: String,
    #[n(1)]
    #[cbor(with = "minicbor::bytes")]
    pub s_data: Vec<u8>,
}

impl ContextSpacial {
    pub fn new(s_ref: String, s_data: Vec<u8>) -> Self {
        Self { s_ref, s_data }
    }
}
