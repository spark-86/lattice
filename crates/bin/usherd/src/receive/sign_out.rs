use anyhow::Result;
use key::enclave::Enclave;
use lattice::{
    Rhex,
    rhex::{
        intent::RhexIntent,
        signature::{RhexSignature, RhexSignatureType},
    },
};

/// # `sign_out(enclave, intents, pk)`
///
/// Basically takes a Vec of RhexIntent and signs over all of them
/// with the same key, and then returns the author-signed Rhex
///
pub fn sign_out(enclave: &Enclave, intents: Vec<RhexIntent>, pk: &[u8; 32]) -> Result<Vec<Rhex>> {
    let mut output = Vec::new();
    for i in intents {
        let mut r = Rhex::new();
        r.intent = i;
        let sig = enclave.sign(pk, &r.get_hash(RhexSignatureType::Author));
        if sig.is_ok() {
            r.sigs.push(RhexSignature {
                pk: pk.clone(),
                sig: sig.unwrap(),
                t: RhexSignatureType::Author,
            });
            output.push(r);
        }
    }

    Ok(output)
}
