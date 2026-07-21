use std::collections::HashMap;

use crate::Usher;

pub type UsherMap = HashMap<[u8; 32], Usher>;

pub fn disk_from(path: &str) -> UsherMap {
    let data = std::fs::read(path).unwrap();
    let ushers: Vec<Usher> = minicbor::decode(&data).unwrap();
    let mut ushers_map = HashMap::new();
    for usher in ushers {
        ushers_map.insert(usher.pk, usher);
    }
    ushers_map
}

pub fn disk_to(path: &str, ushers: UsherMap) {
    let mut ushers_vec = Vec::new();
    for (_, usher) in ushers {
        ushers_vec.push(usher);
    }
    let data = minicbor::to_vec(&ushers_vec).unwrap();
    std::fs::write(path, &data).unwrap();
}
