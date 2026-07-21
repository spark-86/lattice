use std::collections::HashMap;

use anyhow::Result;
use minicbor::{Decode, Encode};

use crate::Scope;

impl Scope {
    pub fn ushers_at(&self, time: u64) -> Result<Vec<([u8; 32], UsherAssignment)>> {
        let mut output = Vec::new();
        for (key, assignment) in &self.ushers {
            let mut assigned = assignment.clone();
            assigned.retain(|ua| ua.issued < time);
            if assigned.last().is_some() {
                let last = assigned.last().unwrap();
                if last.eff < time && last.exp > time {
                    output.push((key.clone(), last.clone()))
                }
            }
        }
        Ok(output)
    }

    pub fn ushers_by_priority(&self, time: u64) -> Result<Vec<([u8; 32], u8)>> {
        let mut curr_ushers = self.ushers_at(time)?;
        curr_ushers.sort_by_key(|&(_, ref asssignment)| asssignment.priority);
        let mut output = Vec::new();
        for usher in curr_ushers {
            output.push((usher.0, usher.1.priority));
        }
        Ok(output)
    }

    pub fn ushers_by_role(&self, time: u64) -> Result<HashMap<UsherRole, [u8; 32]>> {
        let mut output = HashMap::new();
        let curr_ushers = self.ushers_at(time)?;
        for usher in curr_ushers {
            for r in usher.1.roles {
                output.insert(r, usher.0);
            }
        }
        Ok(output)
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct UsherAssignment {
    #[n(0)]
    pub issued: u64,
    #[n(1)]
    pub priority: u8,
    #[n(2)]
    pub roles: Vec<UsherRole>,
    #[n(3)]
    pub eff: u64,
    #[n(4)]
    pub exp: u64,
    #[n(5)]
    #[cbor(with = "minicbor::bytes")]
    pub by: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, Hash)]
pub enum UsherRole {
    #[n(0)]
    Actor,
    #[n(1)]
    Mirror,
    #[n(2)]
    Cache,
    #[n(3)]
    Quorum,
    #[n(4)]
    Observer,
    #[n(5)]
    Other,
}
