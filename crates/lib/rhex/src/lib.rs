use ed25519_dalek::{Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};

/*
    Structure: Rhex

    This is the heart of the whole lattice. This is what each record
    is composed of.

    magic: unique "version" number of the record. We've made a few
        revisions over time, and this is the current implementation.
        As no ushers for earlier versions were completed this is
        techically the first implemented version @ ver 2
    intent: This is what the author signs over and submits to the
        indicated usher. See intent.rs for more details.
    context: Applied by the receiving usher and used as the telemetry
        metric used in quorum. See context.rs for more details.
    sigs: An ordered array of RhexSignature. Author = 0, usher = 1,
        quorum and observers = 2..n
    curr: The final current hash after the author has assembled the
        quorum responses and seals it for final approval to the usher.
*/
use crate::{
    context::RhexContext,
    intent::RhexIntent,
    signature::{RhexSignature, RhexSignatureType},
};

pub mod build;
pub mod context;
pub mod data;
pub mod data_bytes;
pub mod intent;
pub mod signature;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rhex {
    pub magic: [u8; 6],
    pub intent: RhexIntent,
    pub context: RhexContext,
    pub sigs: Vec<RhexSignature>,
    pub curr: Option<[u8; 32]>,
}

impl Rhex {
    pub fn new() -> Self {
        Self {
            magic: *b"RHEX\x00\x02",
            intent: RhexIntent::new(),
            context: RhexContext { at: 0, s: None },
            sigs: Vec::new(),
            curr: None,
        }
    }

    pub fn get_hash(&self, sig_type: RhexSignatureType) -> [u8; 32] {
        match sig_type {
            RhexSignatureType::Author => {
                let mut hasher = blake3::Hasher::new();
                hasher.update(b"RHEX_AUTHOR_SIG_0");
                hasher.update(&self.intent.get_hash().clone());
                hasher.finalize().into()
            }
            RhexSignatureType::Usher => {
                let mut hasher = blake3::Hasher::new();
                hasher.update(b"RHEX_USHER_SIG_0");
                hasher.update(&self.sigs[0].sig);
                hasher.update(&self.context.get_hash());
                hasher.finalize().into()
            }
            RhexSignatureType::Quorum | RhexSignatureType::Observer | RhexSignatureType::Other => {
                let mut hasher = blake3::Hasher::new();
                hasher.update(b"RHEX_OBSERVED_SIG_0");
                hasher.update(&self.sigs[0].sig);
                hasher.update(&self.sigs[1].sig);
                hasher.finalize().into()
            }
        }
    }

    /// Sorts signatures 2..n
    /// First two are fixed (Author, Usher), but the rest are stored
    /// byte sorted by the signature itself.
    pub fn sort_sigs(&mut self) {
        let author = self.sigs.remove(0);
        let usher = self.sigs.remove(0);
        self.sigs.sort_by(|a, b| a.sig.cmp(&b.sig));
        self.sigs.insert(0, author);
        self.sigs.insert(1, usher);
    }

    pub fn to_vec(&self) -> Vec<u8> {
        serde_cbor::to_vec(&self).unwrap()
    }

    pub fn from_vec(data: Vec<u8>) -> Self {
        serde_cbor::from_slice(&data).unwrap()
    }

    /// Calculates the current hash of the record
    /// H(label | intent | context | sigs)
    pub fn calc_curr(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"RHEX_CURR_HASH_0");
        hasher.update(&self.intent.get_hash());
        hasher.update(&self.context.get_hash());
        hasher.update(&serde_cbor::to_vec(&self.sigs).unwrap());
        hasher.finalize().into()
    }

    pub fn disk_get(path: &str) -> Self {
        let data = std::fs::read(path).unwrap();
        Self::from_vec(data)
    }

    pub fn disk_put(&self, path: &str) {
        std::fs::write(path, &self.to_vec()).unwrap();
    }

    pub fn validate_sig(&self, sig_type: RhexSignatureType, pos: usize) -> bool {
        let hash = self.get_hash(sig_type);
        let sig = ed25519_dalek::Signature::from_bytes(&self.sigs[pos].sig);
        let pk = match VerifyingKey::from_bytes(&self.sigs[pos].pk) {
            Ok(pk) => pk,
            Err(e) => {
                println!("Invalid public key type: {:?}", e);
                return false;
            }
        };
        match pk.verify(&hash, &sig) {
            Ok(_) => true,
            Err(e) => {
                println!("Invalid signature: {:?}", e);
                false
            }
        }
    }

    pub fn validate_curr(&self) -> bool {
        if self.curr.is_none() {
            return false;
        }
        let hash = self.calc_curr();
        hash == self.curr.unwrap()
    }

    pub fn validate(&self) -> bool {
        let mut valid = true;
        for i in 0..self.sigs.len() {
            let sig_ok = self.validate_sig(self.sigs[i].t.clone(), i);
            valid = valid && sig_ok;
        }
        valid = valid && self.validate_curr();
        valid
    }
}
