use super::*;

#[derive(Clone, Debug)]
pub struct Transaction<'a> {
    common_fields: CommonTransactionFields<'a>,
    asset: MorpheusAsset,
}

impl<'a> Transaction<'a> {
    pub fn new(
        common_fields: CommonTransactionFields<'a>, operation_attempts: Vec<OperationAttempt>,
    ) -> Self {
        Self { common_fields, asset: MorpheusAsset { operation_attempts } }
    }

    pub fn fee(&self) -> u64 {
        self.asset.fee()
    }
}

impl<'a> Aip29Transaction for Transaction<'a> {
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
