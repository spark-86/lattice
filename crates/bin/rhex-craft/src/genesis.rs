use std::time;

use anyhow::Result;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use key::enclave::Enclave;
use scope::rhex::{
    self,
    data::RhexData,
    signature::{RhexSignature, RhexSignatureType},
};
use serde_json::json;

pub fn genesis(key: String, enclave_path: String, output: String) -> Result<()> {
    let mut rhex = rhex::Rhex::new();
    //let key = key::Key::disk_get(&key);
    let key_conv: [u8; 32] = URL_SAFE_NO_PAD.decode(key)?.try_into().unwrap();
    let mut enclave = Enclave::new(Some(enclave_path));
    let _ = enclave.populate();

    // Build intent
    rhex.intent.gen_nonce();
    rhex.intent.prev = None;
    rhex.intent.scope = "".to_string();
    rhex.intent.author = key_conv.clone();
    rhex.intent.usher = key_conv.clone();
    rhex.intent.rt = "lattice:genesis".to_string();
    rhex.intent.schema = None;
    rhex.intent.data = RhexData::Mixed {
        meta: json!({
            "at": time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs(),
        }),
        binary: vec![key_conv.clone().to_vec()],
    };

    // Add context since we are the usher too
    rhex.context.at = 0;
    rhex.context.s = None;

    // Sign it like the dirty hoe we are.
    rhex.sigs.push(RhexSignature {
        pk: key_conv.clone(),
        sig: enclave.sign(&key_conv, &rhex.get_hash(RhexSignatureType::Author))?,
        t: RhexSignatureType::Author,
    });
    rhex.sigs.push(RhexSignature {
        pk: key_conv.clone(),
        sig: enclave.sign(&key_conv, &rhex.get_hash(RhexSignatureType::Usher))?,
        t: RhexSignatureType::Usher,
    });
    rhex.sigs.push(RhexSignature {
        pk: key_conv.clone(),
        sig: enclave.sign(&key_conv, &rhex.get_hash(RhexSignatureType::Quorum))?,
        t: RhexSignatureType::Quorum,
    });

    // Mark us complete
    rhex.curr = Some(rhex.calc_curr());

    rhex.disk_put(&output);
    Ok(())
}
