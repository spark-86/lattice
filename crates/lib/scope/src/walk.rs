use anyhow::{Result, bail};
use rhex::{Rhex, check::CheckStatus};

use crate::{Scope, manage::ScopeUpdateStatus, membership::Membership, policy::Policy};

impl Scope {
    /// # walk(self, init_key, rhex)
    ///
    /// This walks a scope and collects all the necessary R⬢ to build
    /// the scope. This is called on startup or anytime we get a new
    /// scope in our cache
    ///
    pub fn walk_rhex(name: &String, init_key: [u8; 32], rhex: &Vec<Rhex>) -> Result<Self> {
        // build the inital Scope
        let mut scope = Scope::new(name, init_key);

        // We set the initial policy. These defaults will set the
        // state for that `lattice:genesis`/`scope:genesis` R⬢
        let policy = if scope.name.as_str() == "" {
            scope.add_membership(
                "world_line_zero".to_string(),
                vec![init_key],
                Membership {
                    issued: 0,
                    eff: 0,
                    exp: 1_000_000_000_000_000,
                    by: init_key,
                },
            )?;
            Policy::default_lattice_policy()
        } else {
            scope.add_membership(
                "creator".to_string(),
                vec![init_key],
                Membership {
                    issued: 0,
                    eff: 0,
                    exp: 1_000_000_000_000_000,
                    by: init_key,
                },
            )?;
            Policy::default_scope_policy()
        };
        scope.policy_map.push((0, policy));

        // If `rhex` has zero R⬢, we're done so return.
        if rhex.len() == 0 {
            return Ok(scope);
        }

        // Now we make sure the first R⬢ is set and signed by the
        // `init_key`, which is either the lattice master key (mine)
        // or the scope creator's.
        let first_author = rhex[0].intent.author;
        if init_key != first_author {
            bail!("Chain doesn't start with the specified author");
        }

        // Walk and validate per R⬢, while building the part of the
        // scope and keys
        let mut pos = 0;
        for r in rhex {
            let status = scope.final_check(r, &r.context.at)?;
            if status[0] != CheckStatus::Success {
                bail!(format!("Position: {}, Error: {:?}", pos, status[0]));
            }
            let result = scope.process_scope_changes(r)?;
            match result {
                ScopeUpdateStatus::Failed(err) => {
                    bail!(format!("Position: {}, Error: {}", pos, err));
                }
                _ => { /* ??? */ }
            }

            // TODO: Validation transforms need to fire and pass. Like,
            // will prolly do once we get the inital appending process
            // working.

            pos += 1;
        }

        Ok(scope)
    }
}
