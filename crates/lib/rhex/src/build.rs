use rand::random;

use crate::{Rhex, context::RhexContext, intent::RhexIntent};

impl<'a> Rhex<'a> {
    pub fn build(
        prev: Option<[u8; 32]>,
        scope: &'a str,
        nonce: Option<[u8; 32]>,
        author: [u8; 32],
        usher: [u8; 32],
        rt: &'a str,
        schema: Option<&'a str>,
        data_hash: Option<[u8; 32]>,
    ) -> Self {
        let nonce = match nonce {
            Some(n) => n,
            None => random(),
        };
        Self {
            magic: Rhex::MAGIC,
            intent: RhexIntent {
                prev,
                scope,
                nonce,
                author,
                usher,
                schema,
                rt,
                data_hash,
            },
            data: crate::data::RhexData::None,
            context: RhexContext { at: 0, s: None },
            sigs: Vec::new(),
            curr: None,
        }
    }
}
