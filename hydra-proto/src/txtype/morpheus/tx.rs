use super::*;

#[derive(Clone, Debug)]
pub struct Transaction {
    common_fields: CommonTransactionFields,
    asset: MorpheusAsset,
}

impl Transaction {
    pub fn new(
        common_fields: CommonTransactionFields, operation_attempts: Vec<OperationAttempt>,
    ) -> Self {
        Self { common_fields, asset: MorpheusAsset { operation_attempts } }
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
