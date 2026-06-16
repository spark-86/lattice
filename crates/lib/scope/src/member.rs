use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub key: [u8; 32],
    pub eff: u64,
    pub exp: u64,
    pub issued: u64,
    pub tags: Vec<String>,
    pub name: Option<String>,
}

impl Member {
    pub fn new(key: [u8; 32]) -> Self {
        Self {
            key,
            eff: 0,
            exp: 1_000_000_000_000,
            issued: 0,
            tags: vec![],
            name: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberRevoke {
    pub key: [u8; 32],
    pub issued: u64,
    // Do we add an effective time?
}
