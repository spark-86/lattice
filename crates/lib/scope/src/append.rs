use anyhow::Result;
use rhex::Rhex;

use crate::Scope;

impl Scope {
    /// # Scope Append
    /// This is the method that we use to add a new record
    /// to the scope. It assumes the Rhex has been at least
    /// verfied.
    pub fn append(&mut self, rhex: &Rhex, rhex_path: &str) -> Result<()> {
        if rhex.curr == None {
            println!("No current hash set for R⬢!");
            return Ok(());
        }
        let curr = rhex.calc_curr();
        let filename = format!("{}/{}.rhex", rhex_path, hex::encode(curr));
        rhex.disk_put(&filename);
        self.add_rhex(rhex.clone());
        self.head = curr.clone();
        Ok(())
    }
}
