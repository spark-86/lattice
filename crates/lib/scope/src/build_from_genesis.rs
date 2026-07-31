/*use std::path::PathBuf;

use anyhow::Result;

use crate::Scope;

impl Scope {
    /// # build_from_genesis
    /// Takes a path, loads the genesis.rhex and starts walking forward
    /// as opposed to `slurp_scope` which builds from the end (which
    /// assumes we have the head hash)
    ///
    pub fn build_from_genesis(path: String) -> Result<Self> {
        // FIXME: This was done when we were loading Rhex by individual
        // files. What we do now is load a Vec<Rhex> and parse that.
        // I should really add this to github but like... eh. lazy. :/
        let mut done = false;
        let genesis = rhex::Rhex::single_disk_get(format!("{}/genesis.rhex", path).as_str());
        let scope_name = &genesis.intent.scope.to_string();
        let mut scope = Scope::new(scope_name, genesis.intent.author.clone());

        let mut next = genesis.curr;
        while !done {
            let scope_dir = match scope_name.as_str() {
                "" => format!("{}/scopes", path),
                _ => format!("{}/scopes/{}", path, scope_name),
            };
            let rhex_path =
                PathBuf::from(format!("{}/{}.rhex", scope_dir, hex::encode(next.unwrap())));
            if rhex_path.exists() {
                let rhex = rhex::Rhex::single_disk_get(&rhex_path.to_str().unwrap());
                let valid = rhex.validate();

                if valid {
                    //scope.add_rhex(rhex.clone());
                    next = rhex.intent.prev;
                } else {
                    anyhow::bail!(
                        "Chain failed validation on {}.rhex",
                        hex::encode(next.unwrap())
                    );
                }
                if next == None {
                    // I guess technically possible? But like... weird
                    // man. You'd literally have to... I dunno mod the CBOR
                    // files outside of the usher?
                    done = true;
                }
                scope.head = Some(rhex.calc_curr());
            } else {
                done = true;
            }
        }
        Ok(scope)
    }
}*/
