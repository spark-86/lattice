use crate::{Scope, policy::Policy};

impl Scope {
    /// # get_policy_at
    /// This returns the current policy at the specified micromark.
    /// This is necessary because a scope accumilates future policies
    /// and we need to know which one is effective at a certain point.
    ///
    pub fn get_policy_at(&self, time: u64) -> Policy {
        let mut latest_policy_time = 0;
        let mut latest_policy = Policy::new("".to_string());
        for key in self.policy_map.clone().into_keys() {
            if time > key.0 && time < key.1 {
                if latest_policy_time < key.0 {
                    latest_policy_time = key.0;
                    latest_policy = self.policy_map.get(&key).unwrap().clone();
                }
            }
        }
        latest_policy
    }

    /// # purge_expired_policy
    /// Dumps any policies that are expired before a certain time.
    ///
    pub fn purge_expired_policy(&mut self, time: u64) {
        self.policy_map.retain(|&k, _| k.1 < time);
    }
}
