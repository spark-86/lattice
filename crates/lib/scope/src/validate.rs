use rhex::Rhex;

use crate::Scope;

impl<'a> Scope<'a> {
    /// # check_nonce_reused
    /// Quickly skims the chain to see if this nonce has been used
    /// before. Returns `true` if it already exists in the scope
    ///
    pub fn check_nonce_reused(&self, nonce: &[u8; 32]) -> bool {
        self.rhex
            .iter()
            .filter(|r| r.intent.nonce == nonce.clone())
            .collect::<Vec<&Rhex>>()
            .len()
            > 0
    }

    /// # latest_time
    /// Returns the latest `context.at` of the last R⬢ in the scope.
    ///
    pub fn latest_time(&self) -> u64 {
        if self.rhex.len() == 0 {
            return 0;
        }
        self.rhex.last().unwrap().context.at
    }
}
