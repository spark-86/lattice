use std::collections::HashMap;

use crate::{
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
pub mod membership;
pub mod policy;
pub mod rule;
pub mod ushers;
pub mod validate;

#[derive(Debug, Clone)]
pub struct Scope<'a> {
    // canonical name of the scope
    pub name: &'a str,
    // policy calculated from Rhex
    pub policy_map: Vec<(u64, Policy)>,
    // groups of members
    pub memberships: HashMap<([u8; 32], &'a str), Vec<Membership>>,
    // ushers in the scope and their priority
    pub ushers: HashMap<[u8; 32], Vec<UsherAssignment>>,
    // Records in the scope
    pub rhex: Vec<Rhex<'a>>,
    // The current hash of the last record in the chain.
    pub head: Option<[u8; 32]>,
    // Last updated
    pub updated: u64,
}

impl<'a> Scope<'a> {
    pub fn new(name: &'a str, creator: [u8; 32]) -> Self {
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
        let default_policy = match name {
            "" => {
                memberships.insert((creator.clone(), "world_line_zero"), vec![ship]);
                Policy::default_lattice_policy()
            }
            _ => Policy::default_scope_policy(),
        };

        let mut new_policy = Vec::new();
        new_policy.push((0, default_policy));
        Self {
            name,
            policy_map: new_policy,
            memberships,
            ushers,
            rhex: vec![],
            head: None,
            updated: 0,
        }
    }

    pub fn add_rhex(&mut self, rhex: Rhex<'a>) {
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
