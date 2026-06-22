use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhexSignature {
    pub pk: [u8; 32],
    #[serde(with = "BigArray")]
    pub sig: [u8; 64],
    pub t: RhexSignatureType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RhexSignatureType {
    Author,
    Usher,
    Quorum,
    Observer,
    Other,
}

impl RhexSignature {
    pub fn print(&self) -> String {
        let pk = URL_SAFE_NO_PAD.encode(self.pk);
        let sig = URL_SAFE_NO_PAD.encode(self.sig);
        let t = match self.t {
            RhexSignatureType::Author => "Author",
            RhexSignatureType::Usher => "Usher",
            RhexSignatureType::Quorum => "Quorum",
            RhexSignatureType::Observer => "Observer",
            RhexSignatureType::Other => "Other",
        };
        format!("{}: [{}]\n\t\t\t{}", t, pk, sig)
    }
}
