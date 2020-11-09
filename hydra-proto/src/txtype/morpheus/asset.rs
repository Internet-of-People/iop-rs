use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MorpheusAsset {
    pub operation_attempts: Vec<OperationAttempt>,
}

impl MorpheusAsset {
    const MAX_ASSET_SIZE: usize = 1024 * 1024;

    const FEE_BYTES_OFFSET: u64 = 15;
    const FEE_FLAKES_PER_BYTES: u64 = 3000;

    pub fn fee(&self) -> u64 {
        let op_attempts_json = serde_json::to_string(&self.operation_attempts)
            .expect("Implementation error: serializing operation attempts must not fail");
        let bytes = (op_attempts_json.len() as u64).checked_add(Self::FEE_BYTES_OFFSET);
        bytes.and_then(|bytes| bytes.checked_mul(Self::FEE_FLAKES_PER_BYTES)).unwrap_or(u64::MAX)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let json_val = serde_json::to_value(self)?;
        let json_str = canonical_json(&json_val)?;
        Ok(json_str.into_bytes())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        ensure!(
            bytes.len() <= Self::MAX_ASSET_SIZE,
            format!("MorpheusAsset max size is {} bytes", Self::MAX_ASSET_SIZE)
        );
        let this: Self = serde_json::from_slice(bytes)?;
        let canonical_bytes = this.to_bytes()?;
        ensure!(
            bytes == canonical_bytes.as_slice(),
            "Attempt to construct MorpheusAsset from non-canonical bytes"
        );
        Ok(this)
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;

    #[test]
    fn canonical_format_works() {
        let canonical_json = r#"{"operationAttempts":[]}"#.as_bytes();
        let asset = MorpheusAsset::from_bytes(canonical_json).unwrap();
        assert!(asset.operation_attempts.is_empty());
    }

    #[test]
    fn non_canonical_format_fails() {
        let canonical_json = r#"{"operationAttempts": []}"#.as_bytes();
        let err = MorpheusAsset::from_bytes(canonical_json).unwrap_err();
        assert_eq!(err.to_string(), "Attempt to construct MorpheusAsset from non-canonical bytes");
    }
}
