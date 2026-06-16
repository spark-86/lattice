use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhexSignature {
    pub pk: [u8; 32],
    #[serde(with = "BigArray")]
    pub sig: [u8; 64],
    pub t: RhexSignatureType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RhexSignatureType {
    Author,
    Usher,
    Quorum,
    Observer,
    Other,
}
