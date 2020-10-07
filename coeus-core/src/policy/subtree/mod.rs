mod expiration;
mod schema;

pub use expiration::*;
pub use schema::*;

use super::*;

pub trait SubtreePolicy {
    fn validate(
        &self, state: &State, policy_domain: &Domain, domain_after_op: &Domain,
    ) -> Result<()>;
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubtreePolicies {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    expiration: Option<ExpirationPolicy>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    schema: Option<SchemaPolicy>,
}

impl SubtreePolicies {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_schema(mut self, schema: Schema) -> Self {
        self.schema = Some(schema.into());
        self
    }
    pub fn with_expiration(mut self, max_expiry: u64) -> Self {
        self.expiration = Some(max_expiry.into());
        self
    }
}

impl<T: SubtreePolicy> SubtreePolicy for Option<T> {
    fn validate(
        &self, state: &State, policy_domain: &Domain, domain_after_op: &Domain,
    ) -> Result<()> {
        if let Some(p) = self {
            p.validate(state, policy_domain, domain_after_op)
        } else {
            Ok(())
        }
    }
}

impl SubtreePolicy for SubtreePolicies {
    fn validate(
        &self, state: &State, policy_domain: &Domain, domain_after_op: &Domain,
    ) -> Result<()> {
        self.expiration.validate(state, policy_domain, domain_after_op)?;
        self.schema.validate(state, policy_domain, domain_after_op)?;
        Ok(())
    }
}
