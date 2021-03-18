use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SchemaPolicy {
    pub schema: Schema,
}

impl From<Schema> for SchemaPolicy {
    fn from(schema: Schema) -> Self {
        Self { schema }
    }
}
