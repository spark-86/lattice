use std::time;

use anyhow::Result;
use scope::rhex::{
    self,
    data::RhexData,
    signature::{RhexSignature, RhexSignatureType},
};
use serde_json::json;

pub fn genesis(key: String, output: String) -> Result<()> {
    let mut rhex = rhex::Rhex::new();
    let key = key::Key::disk_get(&key);
    if key.pk.is_none() {
        anyhow::bail!("Key not found");
    }

    // Build intent
    rhex.intent.gen_nonce();
    rhex.intent.prev = None;
    rhex.intent.scope = "".to_string();
    rhex.intent.author = key.pk.clone().unwrap();
    rhex.intent.usher = key.pk.clone().unwrap();
    rhex.intent.rt = "lattice:genesis".to_string();
    rhex.intent.schema = None;
    rhex.intent.data = RhexData::Mixed {
        meta: json!({
            "at": time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs(),
        }),
        binary: vec![key.clone().to_vec()],
    };

    // Add context since we are the usher too
    rhex.context.at = 0;
    rhex.context.s = None;

    // Sign it like the dirty hoe we are.
    rhex.sigs.push(RhexSignature {
        pk: key.pk.clone().unwrap(),
        sig: key.sign(&rhex.get_hash(RhexSignatureType::Author)),
        t: RhexSignatureType::Author,
    });
    rhex.sigs.push(RhexSignature {
        pk: key.pk.clone().unwrap(),
        sig: key.sign(&rhex.get_hash(RhexSignatureType::Usher)),
        t: RhexSignatureType::Usher,
    });
    rhex.sigs.push(RhexSignature {
        pk: key.pk.clone().unwrap(),
        sig: key.sign(&rhex.get_hash(RhexSignatureType::Quorum)),
        t: RhexSignatureType::Quorum,
    });

    // Mark us complete
    rhex.curr = Some(rhex.calc_curr());

    rhex.disk_put(&output);
    Ok(())
}
