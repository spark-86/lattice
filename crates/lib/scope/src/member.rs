use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub key: [u8; 32],
    pub tags: Vec<String>,
    pub name: Option<String>,
}

impl Member {
    pub fn new(key: [u8; 32]) -> Self {
        Self {
            key,
            tags: vec![],
            name: None,
        }
    }
}
