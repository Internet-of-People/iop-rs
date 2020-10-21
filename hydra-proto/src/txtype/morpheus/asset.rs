use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MorpheusAsset {
    pub operation_attempts: Vec<OperationAttempt>,
}

impl MorpheusAsset {
    const FEE_BYTES_OFFSET: u64 = 15;
    const FLAKES_PER_BYTES: u64 = 3000;

    pub fn fee(&self) -> u64 {
        let op_attempts_json = serde_json::to_string(&self.operation_attempts)
            .expect("Implementation error: serializing operation attempts must not fail");
        let bytes = (op_attempts_json.len() as u64).checked_add(Self::FEE_BYTES_OFFSET);
        bytes.and_then(|bytes| bytes.checked_mul(Self::FLAKES_PER_BYTES)).unwrap_or(u64::MAX)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let json_val = serde_json::to_value(self)?;
        let json_str = canonical_json(&json_val)?;
        Ok(json_str.into_bytes())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(serde_json::from_slice(bytes)?)
    }
}
