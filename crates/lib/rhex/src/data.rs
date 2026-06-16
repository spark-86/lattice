use crate::data_bytes;
use serde::{Deserialize, Serialize};
use serde_bytes;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "data_type", rename_all = "lowercase")]
pub enum RhexData {
    None,
    Json(serde_json::Value),
    Binary {
        #[serde(with = "serde_bytes")]
        data: Vec<u8>,
    },
    Mixed {
        meta: serde_json::Value,
        #[serde(with = "data_bytes")]
        binary: Vec<Vec<u8>>,
    },
}
