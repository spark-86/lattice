use anyhow::Result;

use crate::Scope;

#[derive(Debug, Clone, PartialEq)]
pub struct Membership {
    pub issued: u64,
    pub eff: u64,
    pub exp: u64,
    pub by: [u8; 32],
}

#[derive(Debug, Clone, PartialEq)]
pub enum MemberAccess {
    Valid,
    MembershipExpired(Membership),
    FutureMembership(Membership),
    NoEntry([u8; 32], String),
}

impl<'a> Scope<'a> {
    /// # membership_status
    /// I used to think this was a good idea, but now I feel like
    /// this is just a lot of work to keep the most current
    /// access rights.
    ///
    pub fn membership_status(&self, key: [u8; 32], group: &str, time: u64) -> MemberAccess {
        let group_map = self.memberships.get(&(key, group));
        if group_map.is_none() {
            return MemberAccess::NoEntry(key, group.to_string());
        };
        let mut group_map = group_map.unwrap().clone();
        group_map.retain(|g| g.issued < time);
        let latest = &group_map.clone()[group_map.len()];
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
    /// group.
    ///
    pub fn add_membership(
        &mut self,
        group: &'a str,
        keys: Vec<[u8; 32]>,
        membership: Membership,
    ) -> Result<()> {
        for key in keys {
            self.memberships
                .insert((key, group), vec![membership.clone()]);
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
        for ((map_key, group), memberships) in &self.memberships {
            if &key == map_key {
                let mut memberships = memberships.clone();
                memberships.retain(|m| m.issued < time);
                if memberships.len() > 0 {
                    let latest = memberships.last().unwrap();
                    if latest.eff < time && latest.exp > time {
                        output.push(group.to_string());
                    }
                }
            }
        }
        Ok(output)
    }
}
