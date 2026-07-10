use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use minicbor::{Decode, Encode};

#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct RhexSignature {
    #[n(0)]
    pub pk: [u8; 32],
    #[n(1)]
    pub sig: [u8; 64],
    #[n(2)]
    pub t: RhexSignatureType,
}

#[derive(Debug, Clone, Copy, PartialEq, Encode, Decode)]
pub enum RhexSignatureType {
    #[n(0)]
    Author,
    #[n(1)]
    Usher,
    #[n(2)]
    Quorum,
    #[n(3)]
    Observer,
    #[n(4)]
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
