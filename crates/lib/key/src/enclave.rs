use std::fs;

use anyhow::{Ok, Result};

use crate::Key;

pub struct Enclave {
    path: String,
    pub keys: Vec<[u8; 32]>,
}

impl Enclave {
    pub fn new(path: Option<String>) -> Self {
        let path = path.unwrap_or("./keys".to_string());

        Self {
            path,
            keys: Vec::new(),
        }
    }

    pub fn disk_get(&self, pk: [u8; 32]) -> Result<Key> {
        let data = std::fs::read(format!("{}/{}.key", self.path, hex::encode(&pk))).unwrap();
        let key: Key = serde_cbor::from_slice::<Key>(&data).unwrap();
        Ok(key)
    }

    pub fn disk_put(&self, pk: [u8; 32]) {
        let key = self.keys.iter().find(|c| **c == pk);
        if key.is_none() {
            return;
        }
        let key = key.unwrap();
        let _ = std::fs::write(
            format!("{}/{}.key", self.path, hex::encode(&pk)),
            &serde_cbor::to_vec(key).unwrap(),
        );
    }

    pub fn populate(&mut self) -> Result<()> {
        if !fs::exists(self.path.clone())? {
            fs::create_dir(self.path.clone()).unwrap();
        }
        let dir = fs::read_dir(self.path.clone())?;
        for entry in dir {
            let key = fs::read(entry.unwrap().path()).unwrap();
            let key: Key = serde_cbor::from_slice(&key).unwrap();
            self.keys.push(key.pk.unwrap());
        }
        Ok(())
    }

    pub fn sign(&self, pk: &[u8; 32], msg: &[u8]) -> Result<[u8; 64]> {
        let key = self.keys.iter().find(|c| *c == pk);
        if key.is_none() {
            return Err(anyhow::anyhow!("Key not found"));
        }

        let key = self.disk_get(*pk)?;
        let sig = key.sign(msg);
        Ok(sig)
    }

    pub fn generate(&mut self, name: Option<String>) -> Result<Key> {
        let mut key = Key::generate(name);
        let data = serde_cbor::to_vec(&key).unwrap();
        std::fs::write(
            format!("{}/{}.key", self.path, hex::encode(&key.pk.unwrap())),
            &data,
        )?;
        key.sk = None;
        self.keys.push(key.pk.unwrap());
        Ok(key)
    }

    pub fn show_key(&self, pk: [u8; 32], show_sk: bool) -> Result<String> {
        let key = self.disk_get(pk)?;
        Ok(key.pretty_format(show_sk))
    }
}
