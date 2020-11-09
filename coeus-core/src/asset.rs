use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoeusAsset {
    pub bundles: Vec<SignedBundle>,
}

// TODO work out ecosystem for pricing model
impl CoeusAsset {
    const MAX_ASSET_SIZE: usize = 1024 * 1024;

    const FEE_BYTES_OFFSET: u64 = 0;
    //const FEE_FLAKES_PER_BYTES: u64 = 3000;

    pub fn fee(&self) -> u64 {
        let price =
            self.bundles.iter().fold(Price::fee(Self::FEE_BYTES_OFFSET), |mut price, bundle| {
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
