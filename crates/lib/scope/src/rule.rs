use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub append: Vec<String>,
    pub k: u16,
    pub quorum: Vec<String>,
    pub delay: u64,
    pub rt: Vec<String>,
    pub window: u64,
}
