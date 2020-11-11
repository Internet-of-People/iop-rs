use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoeusAsset {
    pub bundles: Vec<SignedBundle>,
}

// TODO Coeus txn builder (or operation builder) should check on client-side whether the asset is canonical
impl CoeusAsset {
    const MAX_ASSET_SIZE: usize = 1024 * 1024;

    const FEE_BYTES_OFFSET: u64 = 0;
    const FEE_FLAKES_PER_BYTES: u64 = 3000;

    // TODO fee calculation could also err, so why is it not a Result<u64>?
    pub fn fee(&self) -> u64 {
        let size_fee =
            Price::fee((Self::FEE_BYTES_OFFSET + self.size()) * Self::FEE_FLAKES_PER_BYTES);
        let price = self.bundles.iter().fold(size_fee, |mut price, bundle| {
            price += bundle.get_price();
            price
        });
        price.fee
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let json_val = serde_json::to_value(self)?;
        let json_str = canonical_json(&json_val)?;
        Ok(json_str.into_bytes())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        ensure!(
            bytes.len() <= Self::MAX_ASSET_SIZE,
            format!("CoeusAsset max size is {} bytes", Self::MAX_ASSET_SIZE)
        );
        let this: Self = serde_json::from_slice(bytes)?;
        let canonical_bytes = this.to_bytes()?;
        ensure!(
            bytes == canonical_bytes.as_slice(),
            "Attempt to construct CoeusAsset from non-canonical bytes"
        );
        Ok(this)
    }

    // TODO We are forced not to err, so we are rather pessimistic if serialization fails
    fn size(&self) -> u64 {
        match serde_json::to_string(self) {
            Ok(json) => {
                // json might be not canonical, but latest when posting the transaction to the mempool,
                // from_bytes will reject not canonical data, so it will not get into the blockchain.
                // This might mislead the client, but a dry-run on a node will report a proper error
                // pointing to a non-canonical asset.
                //
                // The calculated fee might be wrong in these cases, but that is the least problem with
                // a non-canonical asset that will be rejected. Extra whitespace, key duplication, and
                // non-NFKD Unicode normalization in schema or resolved data are the possible causes for
                // these mistakes.
                json.len() as u64
            }
            Err(_) => Self::MAX_ASSET_SIZE as u64,
        }
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;

    #[test]
    fn canonical_format_works() {
        let canonical_json = r#"{"bundles":[]}"#.as_bytes();
        let asset = CoeusAsset::from_bytes(canonical_json).unwrap();
        assert!(asset.bundles.is_empty());
    }

    #[test]
    fn non_canonical_format_fails() {
        let canonical_json = r#"{"bundles": []}"#.as_bytes();
        let err = CoeusAsset::from_bytes(canonical_json).unwrap_err();
        assert_eq!(err.to_string(), "Attempt to construct CoeusAsset from non-canonical bytes");
    }
}
