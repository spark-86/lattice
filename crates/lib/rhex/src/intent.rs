/*
    Structure: RhexIntent

    This is the core of what the author proposes. Only the author
    gets to set these values, so head lookup is important before
    signing because the "prev"ious hash can't be set after the fact.

    prev: the "curr"ent hash of the last record in the chain
    scope: The namespace this record lives in
    nonce: rando generated
    author: Public key of the author
    usher: Public key of the usher
    schema: lattice referrenced (rhex://) or internet
        referrenced (https://)
    rt: Record type, eg, "policy:set", "key:grant"
    data: Enum of the possible payload types. See data.rs for more
        details.
*/
use serde::{Deserialize, Serialize};

use crate::data::RhexData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhexIntent {
    pub prev: Option<[u8; 32]>,
    pub scope: String,
    pub nonce: [u8; 32],
    pub author: [u8; 32],
    pub usher: [u8; 32],
    pub schema: Option<String>,
    pub rt: String,
    pub data: RhexData,
}

impl RhexIntent {
    pub fn new() -> Self {
        Self {
            prev: None,
            scope: String::new(),
            nonce: [0; 32],
            author: [0; 32],
            usher: [0; 32],
            schema: None,
            rt: String::new(),
            data: RhexData::None,
        }
    }

    pub fn get_hash(&self) -> [u8; 32] {
        blake3::hash(&serde_cbor::to_vec(&self).unwrap()).into()
    }

    pub fn gen_nonce(&mut self) {
        self.nonce = rand::random();
    }

    pub fn build(
        prev: Option<[u8; 32]>,
        scope: String,
        author: [u8; 32],
        usher: [u8; 32],
        schema: Option<String>,
        rt: String,
        data: RhexData,
    ) -> Self {
        let mut intent = Self::new();
        intent.prev = prev;
        intent.scope = scope;
        intent.author = author;
        intent.usher = usher;
        intent.gen_nonce();
        intent.schema = schema;
        intent.rt = rt;
        intent.data = data;
        intent
    }
}
