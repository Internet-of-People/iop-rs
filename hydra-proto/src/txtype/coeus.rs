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
        let asset_json = serde_json::to_string(self)?;
        IopAsset::string_to_protobuf(&asset_json)
    }
}
