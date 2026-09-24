use anyhow::Result;
use minicbor::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode)]
pub struct Rule {
    #[n(0)]
    pub append: Vec<String>,
    #[n(1)]
    pub k: u16,
    #[n(2)]
    pub o: u16,
    #[n(3)]
    pub quorum: Vec<String>,
    #[n(4)]
    pub delay: u64,
    #[n(5)]
    pub rt: Vec<String>,
    #[n(6)]
    pub window: u64,
}

impl Rule {
    pub fn valid(&self, rt: &String, groups: &Vec<String>) -> Result<bool> {
        let mut valid = false;
        for r in self.rt.clone() {
            if r.ends_with(":*") {
                let parent: Vec<&str> = r.split(":").collect();
                let rt_parent: Vec<&str> = rt.split(":").collect();
                if parent[0] == rt_parent[0] {
                    for g in groups {
                        if self.append.contains(g) {
                            valid = true;
                        }
                    }
                }
            }
        }
        Ok(valid)
    }
}
