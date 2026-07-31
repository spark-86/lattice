use std::{fs, path::PathBuf};

use anyhow::Result;

use crate::Rhex;

impl Rhex {
    pub fn chain_from_disk(path: PathBuf) -> Result<Vec<Rhex>> {
        let file = fs::read(path)?;
        let rhex: Vec<Rhex> = minicbor::decode(&file)?;
        Ok(rhex)
    }

    pub fn chain_to_disk(path: PathBuf, rhex: Vec<Rhex>) -> Result<()> {
        let mut bin = Vec::new();
        minicbor::encode(rhex, &mut bin)?;
        fs::write(path, bin)?;
        Ok(())
    }
}
