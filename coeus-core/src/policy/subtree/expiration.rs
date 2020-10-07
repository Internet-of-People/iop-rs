use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExpirationPolicy {
    max_expiry: u64,
}

impl ExpirationPolicy {
    pub const YEAR: u64 = 2_628_000;
}

impl From<u64> for ExpirationPolicy {
    fn from(max_expiry: u64) -> Self {
        Self { max_expiry }
    }
}

impl SubtreePolicy for ExpirationPolicy {
    fn validate(
        &self, state: &State, policy_domain: &Domain, domain_after_op: &Domain,
    ) -> Result<()> {
        if let Some(policy_expiration) = &policy_domain.subtree_policies().expiration {
            if let Some(checked_expiration) = &domain_after_op.subtree_policies().expiration {
                ensure!(
                    policy_expiration.max_expiry <= checked_expiration.max_expiry,
                    "Cannot make expiration of {} longer than what {} defined",
                    policy_domain.name(),
                    domain_after_op.name()
                );
            }
            ensure!(
                state.last_seen_height() < domain_after_op.expires_at_height(),
                "Domain {} expired",
                domain_after_op.name()
            );
            ensure!(
                state.last_seen_height() + policy_expiration.max_expiry
                    >= domain_after_op.expires_at_height(),
                "Domain {} would expire too late based on policy in {}",
                domain_after_op.name(),
                policy_domain.name()
            );
        }
        Ok(())
    }
}
