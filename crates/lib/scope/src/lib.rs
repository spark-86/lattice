use std::collections::HashMap;

use crate::{
    membership::Membership,
    policy::Policy,
    ushers::{UsherAssignment, UsherRole},
};

pub use rhex;

pub mod build_from_genesis;
pub mod can_submit;
pub mod get_policy_at;
pub mod membership;
pub mod policy;
pub mod rule;
pub mod ushers;
pub mod validate;

#[derive(Debug, Clone)]
pub struct Scope {
    // canonical name of the scope
    pub name: String,
    // policy calculated from Rhex
    pub policy_map: Vec<(u64, Policy)>,
    // groups of members
    pub memberships: HashMap<([u8; 32], String), Vec<Membership>>,
    // ushers in the scope and their priority
    pub ushers: HashMap<[u8; 32], Vec<UsherAssignment>>,
    // The current hash of the last record in the chain.
    pub head: Option<[u8; 32]>,
    // Last updated
    pub updated: u64,
}

impl Scope {
    pub fn new(name: &String, creator: [u8; 32]) -> Self {
        let ship = Membership {
            issued: 0,
            eff: 0,
            exp: 1_000_000_000_000_000,
            by: creator.clone(),
        };
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
        let default_policy = match name.as_str() {
            "" => {
                memberships.insert((creator.clone(), "world_line_zero".to_string()), vec![ship]);
                Policy::default_lattice_policy()
            }
            _ => Policy::default_scope_policy(),
        };

        let mut new_policy = Vec::new();
        new_policy.push((0, default_policy));
        Self {
            name: name.to_string(),
            policy_map: new_policy,
            memberships,
            ushers,
            head: None,
            updated: 0,
        }
    }

    pub fn slurp_scope(&mut self, _path_prefix: String) -> Self {
        // TODO: Redo without packing the Rhex to the scope, that
        // functionality has been removed
        /*let path = format!("{}/{}", path_prefix, self.name);
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
        }*/
        self.clone()
    }
}
