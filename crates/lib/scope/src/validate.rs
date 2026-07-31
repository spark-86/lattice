use rhex::Rhex;

use crate::Scope;

impl Scope {
    /// # check_nonce_reused
    /// Quickly skims the chain to see if this nonce has been used
    /// before. Returns `true` if it already exists in the scope
    ///
    pub fn check_nonce_reused(&self, nonce: [u8; 32], rhex: &Vec<Rhex>) -> bool {
        // TODO: Fix this so it uses the Rhex from disk because
        // we no longer store them in the Scope itself.
        rhex.iter()
            .filter(|r| r.intent.nonce == nonce.clone())
            .collect::<Vec<&Rhex>>()
            .len()
            > 0
    }

    /// # latest_time
    /// Returns the latest `context.at` of the last R⬢ in the scope.
    /// FIXME: This just returns `self.updated` which SHOULD be set to
    /// the latest record, unless somehow there was a modification
    /// outside the chain, which we don't allow, but it still feels wrong
    /// blindly pulling here
    ///
    pub fn latest_time(&self) -> u64 {
        self.updated
    }
}
