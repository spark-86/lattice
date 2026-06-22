use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{member::Member, policy::Policy, rhex::Rhex};

pub use rhex;

pub mod append;
pub mod build_from_genesis;
pub mod can_submit;
pub mod filter;
pub mod key;
pub mod member;
pub mod policy;
pub mod rule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scope {
    // canonical name of the scope
    pub name: String,
    // policy calculated from Rhex
    pub policy: Policy,
    // groups of members
    pub groups: HashMap<String, Vec<[u8; 32]>>,
    // members of groups
    pub members: HashMap<[u8; 32], Member>,
    // ushers in the scope and their priority
    pub ushers: Vec<([u8; 32], u16)>,
    // Records in the scope
    pub rhex: Vec<Rhex>,
    // The current hash of the last record in the chain.
    pub head: [u8; 32],
    // Last updated
    pub updated: u64,
}

impl Scope {
    pub fn new(name: String, creator: Option<[u8; 32]>) -> Self {
        let (default_policy, groups, members, usher_key) = match name.as_str() {
            "" => {
                let mut new_groups = HashMap::new();
                new_groups.insert("world_line_zero".to_string(), vec![creator.unwrap()]);
                let mut new_members = HashMap::new();
                new_members.insert(
                    creator.unwrap(),
                    Member {
                        key: creator.unwrap(),
                        eff: 0,
                        exp: 1_000_000_000_000,
                        issued: 0,
                        tags: vec!["wl0".to_string()],
                        name: None,
                    },
                );
                (
                    Policy::default_lattice_policy(),
                    new_groups,
                    new_members,
                    creator.unwrap(),
                )
            }
            _ => {
                let mut new_groups: HashMap<String, Vec<[u8; 32]>> = HashMap::new();
                new_groups.insert("creator".to_string(), vec![creator.unwrap()]);
                let mut new_members = HashMap::new();
                new_members.insert(
                    creator.unwrap(),
                    Member {
                        key: creator.unwrap(),
                        eff: 0,
                        exp: 1_000_000_000_000,
                        issued: 0,
                        tags: vec!["creator".to_string()],
                        name: Some("creator".to_string()),
                    },
                );
                (
                    Policy::default_scope_policy(),
                    new_groups,
                    new_members,
                    creator.unwrap(),
                )
            }
        };
        Self {
            name,
            policy: default_policy,
            groups,
            members,
            ushers: vec![(usher_key, 0)],
            rhex: vec![],
            head: [0; 32],
            updated: 0,
        }
    }

    pub fn add_usher(&mut self, usher: [u8; 32], priority: u16) {
        self.ushers.push((usher, priority));
    }

    pub fn drop_usher(&mut self, pk: [u8; 32]) {
        self.ushers.retain(|usher| usher.0 != pk);
    }

    pub fn add_rhex(&mut self, rhex: Rhex) {
        self.rhex.push(rhex);
    }

    pub fn slurp_scope(&mut self, path_prefix: String) -> Self {
        let path = format!("{}/{}", path_prefix, self.name);
        let mut done = false;
        let mut next: Option<[u8; 32]> = Some(self.head);
        while !done {
            let rhex_path = format!("{}/{}.rhex", path, hex::encode(next.unwrap()));
            let rhex = rhex::Rhex::disk_get(&rhex_path);
            self.add_rhex(rhex.clone());
            next = rhex.intent.prev;
            if next == None {
                if rhex.intent.rt.ends_with("genesis") {
                    done = true;
                } else {
                    println!(
                        "First record isn't a *:genesis, which should always be the case (see https://trust.archi/firstrecord)"
                    );
                }
            }
        }
        self.clone()
    }
}
