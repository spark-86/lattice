use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    member::Member,
    membership::Membership,
    policy::Policy,
    rhex::Rhex,
    ushers::{UsherAssignment, UsherRole},
};

pub use rhex;

pub mod build_from_genesis;
pub mod can_submit;
pub mod filter;
pub mod get_policy_at;
pub mod member;
pub mod membership;
pub mod policy;
pub mod rule;
pub mod ushers;
pub mod validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scope {
    // canonical name of the scope
    pub name: String,
    // policy calculated from Rhex
    pub policy_map: HashMap<(u64, u64), Policy>,
    // groups of members
    pub memberships: HashMap<String, HashMap<[u8; 32], Vec<Membership>>>,
    // members of groups
    pub members: HashMap<[u8; 32], Member>,
    // ushers in the scope and their priority
    pub ushers: HashMap<[u8; 32], Vec<UsherAssignment>>,
    // Records in the scope
    pub rhex: Vec<Rhex>,
    // The current hash of the last record in the chain.
    pub head: Option<[u8; 32]>,
    // Last updated
    pub updated: u64,
}

impl Scope {
    pub fn new(name: String, creator: [u8; 32]) -> Self {
        let ship = Membership {
            issued: 0,
            eff: 0,
            exp: 1_000_000_000_000_000,
            by: creator.clone(),
        };
        let mut assignments = HashMap::new();
        let mut memberships = HashMap::new();
        let mut ushers = HashMap::new();
        ushers.insert(
            creator.clone(),
            vec![UsherAssignment {
                issued: 0,
                priority: 0,
                roles: vec![UsherRole::Actor],
                eff: 0,
                exp: 1_000_000_000_000_000,
                by: creator.clone(),
            }],
        );
        let mut members = HashMap::new();
        let default_policy = match name.as_str() {
            "" => {
                assignments.insert(creator.clone(), vec![ship]);
                memberships.insert("world_line_zero".to_string(), assignments);
                members.insert(
                    creator.clone(),
                    Member {
                        key: creator.clone(),
                        tags: vec!["wl0".to_string()],
                        name: Some("Veronica Hodo".to_string()),
                    },
                );
                Policy::default_lattice_policy()
            }
            _ => Policy::default_scope_policy(),
        };

        let mut new_policy = HashMap::new();
        new_policy.insert((0, 1_000_000_000_000), default_policy);
        Self {
            name,
            policy_map: new_policy,
            memberships,
            members,
            ushers,
            rhex: vec![],
            head: None,
            updated: 0,
        }
    }

    pub fn add_rhex(&mut self, rhex: Rhex) {
        self.rhex.push(rhex);
    }

    pub fn slurp_scope(&mut self, path_prefix: String) -> Self {
        let path = format!("{}/{}", path_prefix, self.name);
        let mut done = false;
        let mut next: Option<[u8; 32]> = self.head;
        while !done {
            let rhex_path = format!("{}/{}.rhex", path, hex::encode(next.unwrap()));
            let rhex = rhex::Rhex::disk_get(&rhex_path);
            self.add_rhex(rhex.clone());
            next = rhex.intent.prev;
            if next == None {
                if rhex.intent.rt.ends_with(":genesis") {
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
