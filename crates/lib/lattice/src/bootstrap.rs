use crate::Lattice;
use anyhow::Result;
use usher;

impl Lattice {
    pub fn bootstrap(&mut self, path: &String) -> Result<()> {
        let ushers = usher::map::disk_from(&format!("{}/ushers.cbor", path));
        self.ushers = ushers;
        Ok(())
    }
}
