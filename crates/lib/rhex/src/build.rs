use anyhow::Result;
use serde_json::json;

use crate::{Rhex, data::RhexData};

impl Rhex {
    pub fn build_error(
        scope: &String,
        author: [u8; 32],
        usher: [u8; 32],
        code: u16,
        msg: &String,
    ) -> Result<Rhex> {
        let mut out = Rhex::new();
        out.intent.gen_nonce();
        out.intent.prev = None;
        out.intent.scope = scope.clone();
        out.intent.author = author;
        out.intent.usher = usher;
        out.intent.rt = "reponse:error".to_string();
        out.intent.data = RhexData::Json(json!({
            "code": code,
            "msg": msg
        }));
        out.intent.schema = Some("rhex://schema.response.error/@0".to_string());

        Ok(out)
    }
}
