use minicbor::{Decode, Encode};

#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct RhexContext<'a> {
    #[n(0)]
    pub at: u64,
    #[n(1)]
    #[cbor(borrow)]
    pub s: Option<ContextSpacial<'a>>,
}

impl<'a> RhexContext<'a> {
    pub fn new(at: u64, s: Option<ContextSpacial<'a>>) -> Self {
        Self { at, s }
    }

    pub fn get_hash(&self) -> [u8; 32] {
        blake3::hash(&minicbor::to_vec(&self).unwrap()).into()
    }
}

#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct ContextSpacial<'a> {
    #[n(0)]
    pub s_ref: &'a str,
    #[n(1)]
    #[cbor(with = "minicbor::bytes")]
    pub s_data: &'a [u8],
}

impl<'a> ContextSpacial<'a> {
    pub fn new(s_ref: &'a str, s_data: &'a [u8]) -> Self {
        Self { s_ref, s_data }
    }
}
