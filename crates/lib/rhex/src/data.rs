use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use minicbor::{Decode, Encode};

#[derive(Debug, Clone, Copy, Encode, Decode)]
pub enum RhexData<'a> {
    #[n(0)]
    None,

    #[n(1)]
    Json(
        #[b(0)]
        #[cbor(with = "minicbor::bytes")]
        &'a [u8],
    ),

    #[n(2)]
    Binary(
        #[b(0)]
        #[cbor(with = "minicbor::bytes")]
        &'a [u8],
    ),

    #[n(3)]
    Mixed {
        #[b(0)]
        #[cbor(with = "minicbor::bytes")]
        meta: &'a [u8],
        #[b(1)]
        #[cbor(with = "minicbor::bytes")]
        binary: &'a [u8],
    },
}

impl<'a> RhexData<'a> {
    pub fn print(&self) -> String {
        match self {
            RhexData::None => "None".to_string(),
            RhexData::Json(json) => serde_json::to_string(json).unwrap(),
            RhexData::Binary(data) => URL_SAFE_NO_PAD.encode(data),
            RhexData::Mixed { meta, binary } => {
                let meta_str = serde_json::to_string(meta).unwrap();
                let binary_str = URL_SAFE_NO_PAD.encode(binary);
                format!("Meta: {}\n\t\tBinary: {}", meta_str, binary_str)
            }
        }
    }
}
