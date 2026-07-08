use anyhow::Result;
use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsherAssignment {
    pub issued: u64,
    pub priority: u8,
    pub roles: Vec<UsherRole>,
    pub eff: u64,
    pub exp: u64,
    pub by: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UsherRole {
    Actor,
    Mirror,
    Cache,
    Quorum,
    Observer,
    Other,
}
