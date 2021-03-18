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

    pub fn new(operation_attempts: Vec<OperationAttempt>) -> Self {
        Self { operation_attempts }
    }

    pub fn fee(&self) -> u64 {
        let len = self
            .to_bytes()
            .expect("Implementation error: serializing operation attempts must not fail")
            .len();
        let bytes_opt = (len as u64).checked_add(Self::FEE_BYTES_OFFSET);
        let fee_opt = bytes_opt.and_then(|bytes| bytes.checked_mul(Self::FEE_FLAKES_PER_BYTES));
        fee_opt.unwrap_or(u64::MAX)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        // NOTE We could not apply canonical_json here, otherwise that would break existing layer1 consensus
        let json = serde_json::to_vec(self)?;
        Ok(json)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        ensure!(
            bytes.len() <= Self::MAX_ASSET_SIZE,
            format!("MorpheusAsset max size is {} bytes", Self::MAX_ASSET_SIZE)
        );
        let this: Self = serde_json::from_slice(bytes)?;
        let roundtrip_bytes = this.to_bytes()?;
        ensure!(
            bytes == roundtrip_bytes.as_slice(),
            "Attempt to construct MorpheusAsset from non-standard field ordering and spacing"
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
        let json = r#"{"operationAttempts": []}"#.as_bytes();
        let err = MorpheusAsset::from_bytes(json).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Attempt to construct MorpheusAsset from non-standard field ordering and spacing"
        );
    }
}
