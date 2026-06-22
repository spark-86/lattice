use std::fs;

use anyhow::Result;
use lattice::{Lattice, Rhex, usher};

/// # Lattice Rebuild
/// A core set of ushers and the current root scope are always
/// provided with the distribution, so this resets everything to
/// that bootstrapped state.
pub fn rebuild(path: String) -> Result<Lattice> {
    let scope_path = format!("{}/scopes", path);
    let usher_map_path = format!("{}/bootstrap/ushers.cbor", path);
    let root_scope_bootstrap = format!("{}/bootstrap/root_scope.cbor", path);

    // First, torch {path}/scopes and {path}/usher_map.cbor
    fs::remove_dir_all(&scope_path)?;
    fs::remove_file(&usher_map_path)?;
    fs::create_dir_all(&scope_path)?;

    // Load the bootstrap data
    let ushers = usher::map::disk_get(&usher_map_path);
    usher::map::disk_put(&usher_map_path, ushers);
    let root_scope = fs::read(&root_scope_bootstrap)?;
    let root_rhex: Vec<Rhex> = serde_cbor::from_slice(&root_scope)?;

    // Save the bootstrapped root scope to {path}/scopes/
    for rhex in &root_rhex {
        rhex.disk_put(&scope_path);
    }

    // Make the root scope an object
    let mut root_scope =
        lattice::scope::Scope::new("".to_string(), Some(lattice::Lattice::GENESIS_KEY));
    // ...and populate it
    root_scope.rhex = root_rhex;
    root_scope.head = root_scope.rhex[root_scope.rhex.len() - 1].calc_curr();

    let mut lattice = lattice::Lattice::new();
    lattice.add_scope(root_scope);
    Ok(lattice)
}
