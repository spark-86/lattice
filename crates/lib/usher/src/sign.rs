use crate::Usher;

impl Usher {
    pub fn sign(&self, hash: &[u8], keypath: String) -> [u8; 64] {
        let key = key::Key::disk_get(&keypath);
        key.sign(hash)
    }
}
