use anyhow::{Result, anyhow};
use scope::Scope;

use crate::Lattice;

impl Lattice {
    pub fn startup(&mut self, path: &String) -> Result<()> {
        let scopes_dir_entries = std::fs::read_dir(path)?;
        for entry in scopes_dir_entries {
            let entry = entry?;
            // skip if its a dir
            if entry.path().is_dir() || !entry.path().ends_with(".rchain") {
                continue;
            }
            print!("\t🌐 Loading scope: {}...", entry.file_name().display());
            let scope_path = entry.path();
            let scope_path = scope_path
                .file_prefix()
                .ok_or(anyhow!("No prefix"))?
                .to_str()
                .ok_or(anyhow!("Failed to convert prefix"))?;
            let mut scope = Scope::new(&scope_path.to_string(), [0; 32]);
            let _scope_rhex = scope.slurp_scope(
                entry
                    .file_name()
                    .to_str()
                    .ok_or(anyhow!("Couldn't convert filename"))?
                    .to_string(),
            )?;
            self.add_scope(&scope);
            println!("done");
        }
        Ok(())
    }
}
