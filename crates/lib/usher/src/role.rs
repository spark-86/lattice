use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UsherRole {
    Actor,
    Mirror,
    Cache,
    Quorum,
    Observer,
    Other,
}
