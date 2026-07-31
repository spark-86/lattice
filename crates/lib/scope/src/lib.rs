use std::{collections::HashMap, path::PathBuf};

use crate::{
    membership::Membership,
    policy::Policy,
    ushers::{UsherAssignment, UsherRole},
};

use anyhow::Result;
pub use rhex;
use rhex::Rhex;

pub mod build_from_genesis;
pub mod can_submit;
pub mod check;
pub mod from_chain;
pub mod get_policy_at;
pub mod membership;
pub mod policy;
pub mod process_key;
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

    /// # slurp_scope(&mut self, path)
    /// This takes a Rhex chain named "{scopes}/{scope_name}.rchain"
    /// and loads in into a Vec.
    ///
    pub fn slurp_scope(&mut self, path: String) -> Result<Vec<Rhex>> {
        Ok(Rhex::chain_from_disk(PathBuf::from(format!(
            "{}{}.rchain",
            path, self.name
        )))?)
    }
}
