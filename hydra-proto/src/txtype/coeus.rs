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
        tx_data.set_type(TransactionType::IoP(IopTransactionType::Coeus));
        tx_data.asset = Some(Asset::Coeus(self.asset.to_owned()));
        tx_data.fee = self.common_fields.calculate_fee(self).to_string();
        tx_data
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoeusAsset {
    pub signed_operations: Vec<SignedOperations>,
}

impl CoeusAsset {
    const FEE_BYTES_OFFSET: u64 = 15;
    const FLAKES_PER_BYTES: u64 = 3000;

    pub fn fee(&self) -> u64 {
        // TODO calculate price based on Coeus prices/fees
        // self.signed_operations.iter().fold(Price::zero(), |price, op| price += op.get_price())

        let op_attempts_json = serde_json::to_string(&self.signed_operations)
            .expect("Implementation error: serializing operation attempts must not fail");
        let bytes = (op_attempts_json.len() as u64).checked_add(Self::FEE_BYTES_OFFSET);
        bytes.and_then(|bytes| bytes.checked_mul(Self::FLAKES_PER_BYTES)).unwrap_or(u64::MAX)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let asset_json = serde_json::to_string(self)?;
        IopTransactionType::string_to_protobuf(&asset_json)
    }
}
