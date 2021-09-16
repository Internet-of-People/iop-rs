use super::*;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BeforeProofHistory {
    pub content_id: String,
    pub exists_from_height: Option<BlockHeight>,
    pub txid: Option<String>,
    pub queried_at_height: BlockHeight,
}
