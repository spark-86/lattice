use crate::Lattice;
use anyhow::Result;
use usher;

impl Lattice {
    pub fn bootstrap(&mut self, path: &String) -> Result<()> {
        let ushers = usher::map::disk_get(&format!("{}/ushers.cbor", path));
        self.ushers = ushers;
        Ok(())
    }
}
