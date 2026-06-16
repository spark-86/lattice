use base64::{Engine, engine};
use ed25519_dalek::{Signature, Signer, Verifier};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Key {
    #[serde(with = "serde_bytes")]
    pub sk: Option<[u8; 32]>,
    #[serde(with = "serde_bytes")]
    pub pk: Option<[u8; 32]>,
    pub name: Option<String>,
}

impl Key {
    pub fn new(sk: [u8; 32], name: Option<String>) -> Self {
        let dalek_sk = ed25519_dalek::SigningKey::from_bytes(&sk);
        let pk = dalek_sk.verifying_key().to_bytes();
        Self {
            sk: Some(sk),
            pk: Some(pk),
            name,
        }
    }

    pub fn new_pk(pk: [u8; 32], name: Option<String>) -> Self {
        Self {
            sk: None,
            pk: Some(pk),
            name,
        }
    }

    pub fn generate() -> Self {
        let seed: [u8; 32] = rand::random();
        let dalek_sk = ed25519_dalek::SigningKey::from_bytes(&seed);
        let pk = dalek_sk.verifying_key().to_bytes();
        Self {
            sk: Some(dalek_sk.to_bytes()),
            pk: Some(pk),
            name: None,
        }
    }

    pub fn sign(&self, msg: &[u8]) -> [u8; 64] {
        let dalek_sk = ed25519_dalek::SigningKey::from_bytes(&self.sk.unwrap());
        dalek_sk.sign(msg).to_bytes()
    }

    pub fn verify(&self, msg: &[u8], sig: &[u8]) -> bool {
        let dalek_pk = ed25519_dalek::VerifyingKey::from_bytes(&self.pk.unwrap()).unwrap();
        let valid = dalek_pk.verify(msg, &Signature::from_bytes(sig.try_into().unwrap()));
        match valid {
            Ok(_) => true,
            Err(e) => {
                println!("Invalid signature: {:?}", e);
                false
            }
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        serde_cbor::to_vec(&self).unwrap()
    }

    pub fn from_vec(data: Vec<u8>) -> Self {
        serde_cbor::from_slice(&data).unwrap()
    }

    pub fn disk_get(path: &str) -> Self {
        let data = std::fs::read(path).unwrap();
        Self::from_vec(data)
    }

    pub fn disk_put(&self, path: &str) {
        std::fs::write(path, &self.to_vec()).unwrap();
    }

    /// This generates the SigilID from the key.
    /// Used in scope determination and easy identification
    pub fn sigid_id(&self) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.pk.unwrap());
        let hash = hasher.finalize();
        let crockford = base32::encode(base32::Alphabet::Crockford, &hash.as_bytes()[0..10]);
        format!(
            "{}-{}-{}-{}",
            &crockford[0..4],
            &crockford[4..8],
            &crockford[8..12],
            &crockford[12..16]
        )
    }

    pub fn pretty_format(&self, show_sk: bool) -> String {
        let mut output = String::new();

        if self.sk.is_some() && show_sk {
            let sk = engine::general_purpose::URL_SAFE_NO_PAD.encode(self.sk.unwrap());
            output = format!("Secret Key: BASE64({})\n", sk);
        };
        if self.pk.is_some() {
            let pk = engine::general_purpose::URL_SAFE_NO_PAD.encode(self.pk.unwrap());
            output = format!(
                "{}Public Key: BASE64({})\nSigilID: 💠{}\n",
                output,
                pk,
                self.sigid_id()
            );
        };
        output
    }
}
