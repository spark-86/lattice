use minicbor::{Decode, Encode};

use crate::location::UsherLocation;
pub use crate::map::UsherMap;

pub mod location;
pub mod map;

#[derive(Debug, Clone, Encode, Decode)]
pub struct Usher {
    #[n(0)]
    pub name: Option<String>,
    #[n(1)]
    #[cbor(with = "minicbor::bytes")]
    pub pk: [u8; 32],
    #[n(2)]
    pub location: UsherLocation,
    #[n(3)]
    pub last_updated: u64,
}

impl Usher {
    pub fn new() -> Self {
        Self {
            name: None,
            pk: [0; 32],
            location: UsherLocation::Unknown,
            last_updated: 0,
        }
    }

    pub fn create(name: Option<String>, pk: [u8; 32], location: UsherLocation, now: u64) -> Self {
        Self {
            name,
            pk,
            location,
            last_updated: now,
        }
    }
}
