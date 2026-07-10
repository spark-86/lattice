use ed25519_dalek::{Verifier, VerifyingKey};

use crate::{Rhex, signature::RhexSignatureType};

impl<'a> Rhex<'a> {
    /// # validate_sig
    /// Checks a single signature against it's expected hash
    ///
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

    pub fn data_size(&self) -> usize {
        minicbor::to_vec(&self.data).unwrap().len()
    }
}
