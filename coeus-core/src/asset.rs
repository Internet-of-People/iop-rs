use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoeusAsset {
    pub bundles: Vec<SignedBundle>,
}

// TODO work out ecosystem for pricing model
impl CoeusAsset {
    const FEE_BYTES_OFFSET: u64 = 0;
    //const FLAKES_PER_BYTES: u64 = 3000;

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
        Ok(serde_json::from_slice(bytes)?)
    }
}
