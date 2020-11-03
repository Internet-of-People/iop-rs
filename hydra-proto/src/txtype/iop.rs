use super::*;

#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[repr(u16)]
pub enum IopTransactionType {
    Morpheus = 1,
    Coeus = 2,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum IopAsset {
    Coeus(CoeusAsset),
    Morpheus(morpheus::MorpheusAsset),
}

impl IopAsset {
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        match self {
            IopAsset::Coeus(coeus_asset) => coeus_asset.to_bytes(),
            IopAsset::Morpheus(morpheus_asset) => morpheus_asset.to_bytes(),
        }
    }
}
