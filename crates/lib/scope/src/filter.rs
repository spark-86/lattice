use anyhow::Result;
use rhex::Rhex;

use crate::Scope;

impl Scope {
    /// # filter_rt
    /// Filters a scope by a set of record types
    ///
    pub fn filter_rt(&self, rhex_set: Vec<Rhex>, record_types: Vec<String>) -> Result<Vec<Rhex>> {
        let mut filtered = Vec::new();
        for rhex in rhex_set.iter() {
            // If we have an exact match
            if record_types.contains(&rhex.intent.rt) {
                filtered.push(rhex.clone());
                continue;
            }
            // If we have a parent wildcard match
            let parent: Vec<&str> = rhex.intent.rt.split(":").collect();
            if record_types.contains(&format!("{}:*", parent[0])) {
                filtered.push(rhex.clone());
            }
        }
        Ok(filtered)
    }

    /// # filter_time
    /// Filters by time
    ///
    pub fn filter_time(&self, rhex_set: Vec<Rhex>, start: u64, end: u64) -> Result<Vec<Rhex>> {
        let mut filtered = Vec::new();
        for rhex in rhex_set.iter() {
            if rhex.context.at >= start && rhex.context.at <= end {
                filtered.push(rhex.clone());
            }
        }
        Ok(filtered)
    }
}
