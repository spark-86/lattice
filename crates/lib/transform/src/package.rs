use std::fs;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::descriptor::TransformDescriptor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformPackage {
    pub magic: [u8; 8],
    pub descriptor: TransformDescriptor,
    pub binary: Vec<u8>,
}

impl TransformPackage {
    pub fn new(descriptor: TransformDescriptor, binary: Vec<u8>) -> Self {
        Self {
            magic: *b"TRANS\x00\x00\x00",
            descriptor,
            binary,
        }
    }

    pub fn disk_get(path: String) -> Result<Self> {
        let file_bin = fs::read(path)?;
        let pkg: TransformPackage = serde_cbor::from_slice(&file_bin)?;

        Ok(pkg)
    }
}
