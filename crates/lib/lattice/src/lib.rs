use std::collections::HashMap;

pub use scope;
pub use scope::rhex;
pub use scope::rhex::Rhex;
pub use usher;

pub mod bootstrap;
pub mod startup;

pub struct Lattice {
    pub scopes: HashMap<String, scope::Scope>,
    pub ushers: usher::UsherMap,
    pub gt: u64,
}

impl Lattice {
    /// # Genesis Master Key
    /// This is the only assumption, other than the bootstrapped
    /// addresses of the root scope ushers.
    pub const GENESIS_KEY: [u8; 32] = [
        159, 1, 126, 60, 238, 78, 235, 65, 8, 72, 1, 195, 236, 183, 156, 73, 84, 207, 169, 168, 47,
        25, 25, 98, 254, 71, 65, 201, 65, 216, 23, 211,
    ];

    pub fn new() -> Self {
        Self {
            scopes: HashMap::new(),
            ushers: HashMap::new(),
            gt: 0,
        }
    }

    pub fn add_scope(&mut self, scope: &scope::Scope) {
        self.scopes.insert(scope.name.clone(), scope.clone());
    }

    pub fn add_usher(&mut self, usher: usher::Usher) {
        self.ushers.insert(usher.pk, usher);
    }
}
