use minicbor::{Decode, Encode};
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

#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct RhexIntent<'a> {
    #[n(0)]
    pub prev: Option<[u8; 32]>,
    #[n(1)]
    pub scope: &'a str,
    #[n(2)]
    pub nonce: [u8; 32],
    #[n(3)]
    pub author: [u8; 32],
    #[n(4)]
    pub usher: [u8; 32],
    #[n(5)]
    pub schema: Option<&'a str>,
    #[n(6)]
    pub rt: &'a str,
    #[n(7)]
    pub data_hash: Option<[u8; 32]>,
}

impl<'a> RhexIntent<'a> {
    pub fn new() -> Self {
        Self {
            prev: None,
            scope: "",
            nonce: [0; 32],
            author: [0; 32],
            usher: [0; 32],
            schema: None,
            rt: "",
            data_hash: None,
        }
    }

    pub fn get_hash(&self) -> [u8; 32] {
        blake3::hash(&minicbor::to_vec(&self).unwrap()).into()
    }

    pub fn gen_nonce(&mut self) {
        self.nonce = rand::random();
    }

    pub fn build(
        prev: Option<[u8; 32]>,
        scope: &'a str,
        author: [u8; 32],
        usher: [u8; 32],
        schema: Option<&'a str>,
        rt: &'a str,
        data_hash: Option<[u8; 32]>,
    ) -> Self {
        let mut intent = Self::new();
        intent.prev = prev;
        intent.scope = scope;
        intent.author = author;
        intent.usher = usher;
        intent.gen_nonce();
        intent.schema = schema;
        intent.rt = rt;
        intent.data_hash = data_hash;
        intent
    }
}
