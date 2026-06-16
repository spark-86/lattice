use serde::{Deserialize, Serialize};

use crate::rule::Rule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub desc: String,
    pub rules: Vec<Rule>,
    pub eff: u64,
    pub exp: u64,
    pub tags: Vec<String>,
    pub issued: u64,
}

impl Policy {
    pub fn new(desc: String) -> Self {
        Self {
            desc,
            rules: vec![],
            eff: 0,
            exp: 0,
            tags: vec![],
            issued: 0,
        }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }

    /// This is the initial policy for the whole lattice.
    /// It dictates my key is the only one to start everything.
    pub fn default_lattice_policy() -> Self {
        Self {
            desc: "Lattice Default Policy".to_string(),
            rules: vec![Rule {
                append: vec!["world_line_zero".to_string()],
                k: 1,
                quorum: vec!["world_line_zero".to_string()],
                delay: 1_000_000_000,
                rt: vec!["policy:set".to_string()],
                window: 1_000,
            }],
            eff: 0,
            exp: 1_000_000_000_000,
            tags: vec!["lattice".to_string(), "bootstrap".to_string()],
            issued: 0,
        }
    }

    /// This is the initial policy for any scope below the root ("").
    /// The key used in the scope:request is automatically assigned
    /// to "creator" group.
    pub fn default_scope_policy() -> Self {
        Self {
            desc: "Scope Default Policy".to_string(),
            rules: vec![Rule {
                append: vec!["creator".to_string()],
                k: 1,
                quorum: vec!["creator".to_string()],
                delay: 1_000_000_000_000,
                rt: vec!["policy:set".to_string()],
                window: 1_000,
            }],
            eff: 0,
            exp: 1_000_000_000_000,
            tags: vec![],
            issued: 0,
        }
    }
}
