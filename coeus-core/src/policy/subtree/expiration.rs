use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExpirationPolicy {
    max_lifetime_blocks: BlockCount,
}

impl ExpirationPolicy {
    pub const YEAR: BlockCount = 2_628_000;
}

impl From<BlockCount> for ExpirationPolicy {
    fn from(max_lifetime_blocks: BlockCount) -> Self {
        Self { max_lifetime_blocks }
    }
}

impl SubtreePolicy for ExpirationPolicy {
    fn validate(
        &self, state: &State, policy_domain: &Domain, domain_after_op: &Domain,
    ) -> Result<()> {
        if let Some(policy_expiration) = &policy_domain.subtree_policies().expiration {
            if let Some(checked_expiration) = &domain_after_op.subtree_policies().expiration {
                ensure!(
                    policy_expiration.max_lifetime_blocks <= checked_expiration.max_lifetime_blocks,
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
                state.last_seen_height() + policy_expiration.max_lifetime_blocks
                    >= domain_after_op.expires_at_height(),
                "Domain {} would expire too late based on policy in {}",
                domain_after_op.name(),
                policy_domain.name()
            );
        }
        Ok(())
    }
}
