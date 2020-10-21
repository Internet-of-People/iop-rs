use super::*;

use iop_coeus_core::*;

#[derive(Clone, Debug)]
pub struct Transaction {
    common_fields: CommonTransactionFields,
    asset: CoeusAsset,
}

impl Transaction {
    pub fn new(
        common_fields: CommonTransactionFields, signed_operations: Vec<SignedOperations>,
    ) -> Self {
        Self { common_fields, asset: CoeusAsset { signed_operations } }
    }

    pub fn fee(&self) -> u64 {
        self.asset.fee()
    }
}

impl Aip29Transaction for Transaction {
    fn fee(&self) -> u64 {
        self.asset.fee()
    }

    fn to_data(&self) -> TransactionData {
        let mut tx_data: TransactionData = self.common_fields.to_data();
        tx_data.typed_asset = self.asset.to_owned().into();
        tx_data.fee = self.common_fields.calculate_fee(self).to_string();
        tx_data
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoeusAsset {
    pub signed_operations: Vec<SignedOperations>,
}

// TODO work out ecosystem for pricing model
impl CoeusAsset {
    const FEE_BYTES_OFFSET: u64 = 0;
    //const FLAKES_PER_BYTES: u64 = 3000;

    pub fn fee(&self) -> u64 {
        let price = self.signed_operations.iter().fold(
            Price::fee(Self::FEE_BYTES_OFFSET),
            |mut price, op| {
                price += op.get_price();
                price
            },
        );
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

#[cfg(test)]
mod test {
    use super::*;
    use iop_keyvault::multicipher::MPrivateKey;
    use iop_keyvault::PrivateKey;

    #[test]
    fn binary_roundtrip() {
        let domain: DomainName = ".schema.test".parse().unwrap();

        let ark_passphrase =
            "scout try doll stuff cake welcome random taste load town clerk ostrich";
        let secp_privkey = SecpPrivateKey::from_ark_passphrase(ark_passphrase).unwrap();
        let privkey = MPrivateKey::from(secp_privkey);

        let op_renew = UserOperation::renew(domain.clone(), 12345);
        let op_transfer =
            UserOperation::transfer(domain, Principal::PublicKey(privkey.public_key()));
        let nonced_ops = NoncedOperations::new(vec![op_renew, op_transfer], 42);
        let signed_ops = nonced_ops.sign(&privkey).unwrap();

        let original_asset = CoeusAsset { signed_operations: vec![signed_ops] };
        let bytes = original_asset.to_bytes().unwrap();
        let loaded_asset = CoeusAsset::from_bytes(&bytes).unwrap();
        assert_eq!(original_asset, loaded_asset);
    }
}
