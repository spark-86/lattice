use std::collections::HashMap;

use crate::Usher;

pub type UsherMap = HashMap<[u8; 32], Usher>;

pub fn disk_get(path: &str) -> UsherMap {
    let data = std::fs::read(path).unwrap();
    let ushers: UsherMap = serde_cbor::from_slice(&data).unwrap();
    ushers
}

pub fn disk_put(path: &str, ushers: UsherMap) {
    let data = serde_cbor::to_vec(&ushers).unwrap();
    std::fs::write(path, &data).unwrap();
}
