use serde::{Deserialize, Serialize};

pub use crate::map::UsherMap;
use crate::{location::UsherLocation, role::UsherRole};

pub mod location;
pub mod map;
pub mod role;
pub mod sign;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usher {
    pub name: Option<String>,
    pub pk: [u8; 32],
    pub roles: Vec<UsherRole>,
    pub location: UsherLocation,
    pub last_updated: u64,
}

impl Usher {
    pub fn new() -> Self {
        Self {
            name: None,
            pk: [0; 32],
            roles: vec![],
            location: UsherLocation::Unknown,
            last_updated: 0,
        }
    }

    pub fn create(
        name: Option<String>,
        pk: [u8; 32],
        roles: Vec<UsherRole>,
        location: UsherLocation,
        now: u64,
    ) -> Self {
        Self {
            name,
            pk,
            roles,
            location,
            last_updated: now,
        }
    }
}
