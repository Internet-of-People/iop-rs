use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExpirationPolicy {
    pub max_lifetime_blocks: BlockCount,
}

impl ExpirationPolicy {
    pub const YEAR: BlockCount = 2_628_000;
}

impl From<BlockCount> for ExpirationPolicy {
    fn from(max_lifetime_blocks: BlockCount) -> Self {
        Self { max_lifetime_blocks }
    }
}
