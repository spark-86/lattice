use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::Scope;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Membership {
    pub issued: u64,
    pub eff: u64,
    pub exp: u64,
    pub by: [u8; 32],
}

#[derive(Debug, Clone, PartialEq)]
pub enum MemberAccess {
    Valid,
    NoGroup(String),
    MembershipExpired(Membership),
    FutureMembership(Membership),
    NoEntry([u8; 32]),
}

impl Scope {
    /// # membership_status
    /// I used to think this was a good idea, but now I feel like
    /// this is just a lot of work to keep the most current
    /// access rights.
    ///
    pub fn membership_status(&self, key: [u8; 32], group: String, time: u64) -> MemberAccess {
        let group_map = self.memberships.get(&group);
        if group_map.is_none() {
            return MemberAccess::NoGroup(group);
        };
        let group_map = group_map.unwrap();
        let key_map: Option<&Vec<Membership>> = group_map.get(&key);
        if key_map.is_none() {
            return MemberAccess::NoEntry(key.clone());
        };
        let latest = &key_map.unwrap().clone()[key_map.unwrap().len()];
        if latest.eff < time {
            if latest.exp > time {
                return MemberAccess::Valid;
            } else {
                return MemberAccess::MembershipExpired(latest.clone());
            }
        } else {
            return MemberAccess::FutureMembership(latest.clone());
        }
    }

    /// # add_membership
    /// This updates the membership status for a set of keys in a
    /// group. TODO: Upgrade this to take an array of groups too
    ///
    pub fn add_membership(
        &mut self,
        group: String,
        keys: Vec<[u8; 32]>,
        membership: Membership,
        author: [u8; 32],
    ) -> Result<()> {
        let group = self.memberships.entry(group).or_insert(HashMap::new());

        for key in keys {
            let entry = group.entry(key.clone()).or_insert(Vec::new());
            entry.push(Membership {
                issued: membership.issued,
                eff: membership.eff,
                exp: membership.exp,
                by: author,
            })
        }
        Ok(())
    }

    /// # member_of_at
    /// Gets a Vec of group names that the key is a member of at the
    /// given time interval.
    ///
    /// I feel like this could probably be optimized...?
    ///
    pub fn member_of_at(&self, key: [u8; 32], time: u64) -> Result<Vec<String>> {
        let mut output = Vec::new();
        for (group, map) in &self.memberships {
            let entry = map.get(&key);
            // Do we have an entry in this group for this key?
            if entry.is_some() {
                let entry = entry.unwrap();
                // This grabs the latest membership issued before `time`
                let latest = entry.iter().filter(|e| e.issued < time).last();
                if latest.is_some() {
                    let latest = latest.unwrap();
                    if latest.eff < time && latest.exp > time {
                        output.push(group.clone());
                    }
                }
            }
        }
        Ok(output)
    }
}
