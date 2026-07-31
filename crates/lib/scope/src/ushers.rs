use std::collections::HashMap;

use anyhow::Result;
use minicbor::{Decode, Encode};
use rhex::{Rhex, data::RhexData};
use serde_json::Value;

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

    pub fn process_usher_assign(&mut self, rhex: &Rhex) -> Result<()> {
        if rhex.intent.rt != "usher:assign".to_string() {
            anyhow::bail!("Incorrect record type")
        }
        let payload: RhexData = minicbor::decode(&rhex.data)?;
        // FIXME: This shits harder that a night of Everclear and Taco Bell.
        // Do not allow all these unwraps!
        let (roles, priority, eff, exp, keys) = match payload {
            RhexData::Mixed { meta, binary } => {
                let meta = serde_json::from_slice::<Value>(&meta)?;
                let out_keys: Vec<[u8; 32]> = minicbor::decode(&binary)?;
                let roles = meta.get("roles").unwrap().clone();
                let roles = roles.as_array().unwrap();
                let priority = meta.get("priority").unwrap().clone();
                let eff = meta.get("eff").unwrap().clone();
                let exp = meta.get("exp").unwrap().clone();
                (
                    roles.clone(),
                    priority.as_u64().unwrap(),
                    eff.as_u64().unwrap(),
                    exp.as_u64().unwrap(),
                    out_keys,
                )
            }
            _ => anyhow::bail!("Wrong data type"),
        };
        let mut roles_array = Vec::new();
        for r in roles {
            match r.as_str().unwrap() {
                "actor" => roles_array.push(UsherRole::Actor),
                "cache" => roles_array.push(UsherRole::Cache),
                "mirror" => roles_array.push(UsherRole::Mirror),
                "observer" => roles_array.push(UsherRole::Observer),
                "other" => roles_array.push(UsherRole::Other),
                "quorum" => roles_array.push(UsherRole::Quorum),
                _ => anyhow::bail!("Invalid role"),
            }
        }
        for key in keys {
            let working = self.ushers.get_mut(&key);
            let assignment = UsherAssignment {
                issued: rhex.context.at.clone(),
                priority: priority.try_into().unwrap(),
                roles: roles_array.clone(),
                eff,
                exp,
                by: rhex.intent.author.clone(),
            };
            if working.is_some() {
                working.unwrap().push(assignment.clone());
            } else {
                self.ushers.insert(key.clone(), vec![assignment]);
            }
        }
        Ok(())
    }

    pub fn process_usher_revoke(&mut self, rhex: &Rhex) -> Result<()> {
        let data: RhexData = minicbor::decode(&rhex.data)?;
        let keys = match data {
            RhexData::Binary(b) => minicbor::decode::<Vec<[u8; 32]>>(&b).unwrap(),
            _ => anyhow::bail!("Wrong data type"),
        };

        for key in keys {
            let working = self.ushers.get_mut(&key);
            let assignment = UsherAssignment {
                issued: rhex.context.at.clone(),
                priority: 255,
                roles: vec![],
                eff: rhex.context.at.clone(),
                exp: 1_000_000_000_000_000,
                by: rhex.intent.author.clone(),
            };
            if working.is_some() {
                working.unwrap().push(assignment.clone());
            } else {
                self.ushers.insert(key.clone(), vec![assignment]);
            }
        }
        Ok(())
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
