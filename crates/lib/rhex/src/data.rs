use crate::data_bytes;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
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

impl RhexData {
    pub fn print(&self) -> String {
        match self {
            RhexData::None => "None".to_string(),
            RhexData::Json(json) => serde_json::to_string(json).unwrap(),
            RhexData::Binary { data } => URL_SAFE_NO_PAD.encode(data),
            RhexData::Mixed { meta, binary } => {
                let meta_str = serde_json::to_string(meta).unwrap();
                let binary_str = binary
                    .iter()
                    .map(|b| URL_SAFE_NO_PAD.encode(b))
                    .collect::<Vec<String>>()
                    .join(",");
                format!("Meta: {}\n\t\tBinary: {}", meta_str, binary_str)
            }
        }
    }
}
