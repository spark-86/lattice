use crate::Scope;

impl Scope {
    /// # check_nonce_reused
    /// Quickly skims the chain to see if this nonce has been used
    /// before. Returns `true` if it already exists in the scope
    ///
    pub fn check_nonce_reused(&self, _nonce: &[u8; 32]) -> bool {
        // TODO: Fix this so it uses the Rhex from disk because
        // we no longer store them in the Scope itself.
        /*self.rhex
            .iter()
            .filter(|r| r.intent.nonce == nonce.clone())
            .collect::<Vec<&Rhex>>()
            .len()
            > 0
        */
        false
    }

    /// # latest_time
    /// Returns the latest `context.at` of the last R⬢ in the scope.
    ///
    pub fn latest_time(&self) -> u64 {
        // TODO: Fix this so that it just holds the latest time in
        // the Scope, since we don't keep the Rhex anymore.
        /*if self.rhex.len() == 0 {
            return 0;
        }
        self.rhex.last().unwrap().context.at
        */
        0
    }
}
