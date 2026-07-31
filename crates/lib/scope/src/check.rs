use anyhow::Result;
use rhex::{Rhex, check::CheckStatus};

use crate::Scope;

impl Scope {
    /// # check_same_scope
    /// Checks to make sure this Rhex is for this Scope.
    ///
    pub fn check_same_scope(&self, rhex: &Rhex) -> Result<CheckStatus> {
        // See if this record is even for this scope
        if self.name != rhex.intent.scope {
            return Ok(CheckStatus::NotThisScope {
                presented: rhex.intent.scope.to_string(),
                expected: self.name.to_string(),
            });
        };
        Ok(CheckStatus::Success)
    }

    /// # check_prev
    /// Makes sure the presented `intent.prev` matches the scope head,
    /// or `curr` of the last Rhex in the Scope
    pub fn check_prev(&self, rhex: &Rhex) -> Result<CheckStatus> {
        // Does previous match head?
        if rhex.intent.prev != self.head {
            return Ok(CheckStatus::PrevHashMismatch {
                presented: rhex.intent.prev,
                expected: self.head,
            });
        };
        Ok(CheckStatus::Success)
    }

    /// # check_nonce
    /// Checks to see if the `intent.nonce` has been reused for this
    /// Scope
    ///
    pub fn check_nonce(self, nonce: [u8; 32], rhex: &Vec<Rhex>) -> Result<CheckStatus> {
        // skim for nonce reuse
        if self.check_nonce_reused(nonce, &rhex) {
            return Ok(CheckStatus::NonceReused);
        };
        Ok(CheckStatus::Success)
    }

    /// # check_rt_access
    /// check to make sure we can append this record as the author
    /// at this coordinate.
    ///    
    pub fn check_rt_access(&self, rhex: &Rhex) -> Result<CheckStatus> {
        let groups = self.member_of_at(rhex.intent.author.clone(), rhex.context.at.clone())?;
        if groups.len() == 0 {
            return Ok(CheckStatus::AccessDenied);
        }
        let policy = self.get_policy_at(rhex.context.at.clone());
        let submittable = policy.can_submit(&rhex.intent.rt, &groups);
        if !submittable {
            return Ok(CheckStatus::RtNotAllowed);
        }
        Ok(CheckStatus::Success)
    }

    /// # check_usher
    /// Currently just checks to see if the Usher is listed as valid
    /// in the Scope. Future this may actually do more.
    ///
    pub fn check_usher(&self, rhex: &Rhex) -> Result<CheckStatus> {
        // see if usher specified is available.
        let mut current_usher = self.ushers_at(rhex.context.at.clone())?;
        current_usher.retain(|u| u.0 == rhex.intent.usher);
        if current_usher.len() == 0 {
            return Ok(CheckStatus::InvalidUsher);
        }
        Ok(CheckStatus::Success)
    }

    /// # check_time_reversal
    /// Basically just makes sure we're not trying to creep in an
    /// 'earlier' Rhex
    ///
    pub fn check_time_reversal(self, rhex: &Rhex) -> Result<CheckStatus> {
        // Have we gone backwards in time?
        let latest = self.latest_time();
        if latest > rhex.context.at {
            return Ok(CheckStatus::TimeReversal {
                presented: rhex.context.at.clone(),
                prev: latest,
            });
        }
        Ok(CheckStatus::Success)
    }

    /// # full_check
    /// Checks all the possible common errors. This basically checks
    /// each Rhex field for validity. (Minus nonce reuse, because that
    /// actually calls for pulling the whole chain in, which is not
    /// something we want on quick checks)
    pub fn full_check(&self, rhex: &Rhex) -> Result<Vec<CheckStatus>> {
        let mut outputs = Vec::new();
        outputs.push(rhex.check_data_size()?);
        outputs.push(rhex.check_schema()?);
        outputs.push(self.check_same_scope(rhex)?);
        outputs.push(self.check_prev(rhex)?);
        outputs.push(self.check_rt_access(rhex)?);
        outputs.push(self.check_usher(rhex)?);
        for i in 0..rhex.sigs.len() {
            outputs.push(rhex.check_sig(i)?);
        }

        outputs.retain(|o| *o != CheckStatus::Success);

        if outputs.len() > 0 {
            Ok(outputs)
        } else {
            Ok(vec![CheckStatus::Success])
        }
    }

    /// # final_check
    /// This is similar to `self.full_check` except this also checks
    /// `rhex.curr` to see if it's correct and the overall record size
    ///
    pub fn final_check(&self, rhex: &Rhex) -> Result<Vec<CheckStatus>> {
        let mut outputs = Vec::new();
        let mut full = self.full_check(rhex)?;
        outputs.append(&mut full);
        outputs.push(rhex.check_curr_hash()?);
        outputs.push(rhex.check_total_size()?);

        outputs.retain(|o| *o != CheckStatus::Success);

        if outputs.len() > 0 {
            Ok(outputs)
        } else {
            Ok(vec![CheckStatus::Success])
        }
    }
}
