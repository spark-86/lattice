use std::fs;

use anyhow::Result;
use lattice::{Lattice, Rhex, usher};

use crate::config::UsherdConfig;

/// # Lattice Rebuild
/// A core set of ushers and the current root scope are always
/// provided with the distribution, so this resets everything to
/// that bootstrapped state.
pub fn rebuild(config: &UsherdConfig) -> Result<Lattice> {
    let scope_path = &config.scopes;
    let usher_map_path = &config.usher_map;
    let root_scope_bootstrap = format!("{}root_scope.cbor", &config.bootstrap);

    // First, torch {path}/scopes and {path}/usher_map.cbor
    fs::remove_dir_all(&scope_path)?;
    fs::remove_file(&usher_map_path)?;
    fs::create_dir_all(&scope_path)?;

    // Load the bootstrap data
    let ushers = usher::map::disk_get(&usher_map_path);
    usher::map::disk_put(&usher_map_path, ushers);
    let root_scope = fs::read(&root_scope_bootstrap)?;
    let root_rhex: Vec<Rhex> = serde_cbor::from_slice(&root_scope)?;

    // Save the bootstrapped root scope to {config.scopes}/
    for rhex in &root_rhex {
        rhex.disk_put(&scope_path);
    }

    // Make the root scope an object
    let mut root_scope = lattice::scope::Scope::new("".to_string(), lattice::Lattice::GENESIS_KEY);
    // ...and populate it
    root_scope.rhex = root_rhex;
    root_scope.head = Some(root_scope.rhex[root_scope.rhex.len() - 1].calc_curr());

    let mut lattice = lattice::Lattice::new();
    lattice.add_scope(root_scope);
    Ok(lattice)
}
