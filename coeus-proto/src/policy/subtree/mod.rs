mod expiration;
mod schema;

pub use expiration::*;
pub use schema::*;

use super::*;

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubtreePolicies {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub expiration: Option<ExpirationPolicy>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub schema: Option<SchemaPolicy>,
}

impl SubtreePolicies {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_schema(mut self, schema: Schema) -> Self {
        self.schema = Some(schema.into());
        self
    }
    pub fn with_expiration(mut self, max_expiry: BlockCount) -> Self {
        self.expiration = Some(max_expiry.into());
        self
    }
}
