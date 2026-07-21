use anyhow::Result;
use lattice::scope::{Scope, ushers::UsherRole};

use crate::IAm;

impl IAm {
    /// # pick(self, scope, time, role)
    /// Ok, so I don't like this. We pick a key based on which one
    /// we have with the highest priority. This will work in most
    /// cases but if someone holds multiple keys on the same scope
    /// it will always pick the one with the lowest priority score.
    ///
    /// This can obviously be abused to front run, along with making
    /// any key past the first one inaccessable.
    ///
    pub fn pick(&self, scope: &Scope, time: u64, role: UsherRole) -> Result<Option<[u8; 32]>> {
        let ushers = scope.ushers_at(time)?;
        let mut master = Vec::new();
        for usher in ushers {
            let pubkey = self.entries.get(&usher.0);
            if pubkey.is_some() {
                if usher.1.roles.contains(&role) {
                    master.push(usher);
                }
            }
        }
        if master.len() > 0 {
            master.sort_by(|a, b| a.1.priority.cmp(&b.1.priority));
            Ok(Some(master[0].0))
        } else {
            Ok(None)
        }
    }
}
