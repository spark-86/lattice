use anyhow::Result;
use scope::Scope;

use crate::Lattice;

impl Lattice {
    pub fn startup(&mut self, path: &String) -> Result<()> {
        let scopes_dir_entries = std::fs::read_dir(path)?;
        for entry in scopes_dir_entries {
            let entry = entry?;
            // skip if its a file
            if !entry.path().is_dir() {
                continue;
            }
            print!("\t🌐 Loading scope: {}...", entry.file_name().display());
            let scope_path = entry.path();
            let scope = Scope::build_from_genesis(scope_path.to_str().unwrap().to_string())?;
            self.add_scope(&scope);
            println!("done");
        }
        Ok(())
    }
}
