use std::{collections::HashMap, fs};

use anyhow::Result;
use lattice::scope::Scope;
use serde::{Deserialize, Serialize};

pub type IAm = HashMap<[u8; 32], IAmMember>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IAmMember {
    pub pk: [u8; 32],
    pub name: Option<String>,
    pub location: IAmLocation,
    pub last_updated: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IAmLocation {
    Local,
    Remote { address: String, port: u16 },
    Unknown,
}

pub fn is_usher(me: &IAm, scope: &Scope, time: u64) -> Result<Option<[u8; 32]>> {
    let ushers = scope.ushers_at(time)?;
    for usher in ushers {
        let result = me.get(&usher.0);
        if result.is_some() {
            return Ok(Some(usher.0));
        }
    }
    Ok(None)
}

pub fn disk_from(path: &String) -> Result<IAm> {
    let cbor = fs::read(path)?;
    let iam: IAm = serde_cbor::from_slice(&cbor)?;
    Ok(iam)
}

pub fn disk_to(me: &IAm, path: &String) -> Result<()> {
    let contents = serde_cbor::to_vec(me)?;
    fs::write(path, contents)?;
    Ok(())
}
