pub mod hyd_core;
pub mod morpheus;

use super::*;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TransactionType {
    Core(hyd_core::HydraTransactionType),
    Morpheus(morpheus::MorpheusTransactionType),
}

impl TransactionType {
    pub fn type_group(self) -> u32 {
        match self {
            Self::Core(_) => hyd_core::HydraTransactionType::TYPE_GROUP,
            Self::Morpheus(_) => morpheus::MorpheusTransactionType::TYPE_GROUP,
        }
    }

    pub fn into_u16(self) -> u16 {
        match self {
            Self::Core(core_type) => core_type as u16,
            Self::Morpheus(morpheus_type) => morpheus_type as u16,
        }
    }
}

// TODO consider using a better programming construction than this Default here
impl Default for TransactionType {
    fn default() -> Self {
        Self::Core(hyd_core::HydraTransactionType::Transfer)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Asset {
    Core(hyd_core::HydraAsset),
    Morpheus(morpheus::MorpheusAsset),
}

pub trait Aip29Transaction {
    fn fee(&self) -> u64;
    fn to_data(&self) -> TransactionData;
}

#[derive(Clone, Debug)]
pub struct CommonTransactionFields {
    pub network: &'static dyn Network<Suite = Secp256k1>,
    pub sender_public_key: SecpPublicKey,
    pub nonce: u64,
    pub amount: u64,
    pub manual_fee: Option<u64>,
    pub vendor_field: Option<String>,
}

impl Default for CommonTransactionFields {
    fn default() -> Self {
        Self {
            network: &hyd::Testnet,
            // hydra gas test address from tutorials
            sender_public_key: "03d4bda72219264ff106e21044b047b6c6b2c0dde8f49b42c848e086b97920adbf"
                .parse()
                .unwrap(), // panics if field is both unspecified and changes format
            nonce: Default::default(),
            amount: Default::default(),
            manual_fee: Default::default(),
            vendor_field: Default::default(),
        }
    }
}

impl CommonTransactionFields {
    pub fn calculate_fee(&self, tx: &dyn Aip29Transaction) -> u64 {
        self.manual_fee.unwrap_or_else(|| tx.fee())
    }

    fn to_data(&self) -> TransactionData {
        let mut tx_data = TransactionData::default();
        tx_data.network = Some(self.network.p2pkh_addr()[0]);
        tx_data.version = Some(2);

        tx_data.sender_public_key = self.sender_public_key.to_string();
        tx_data.nonce = Some(self.nonce.to_string());
        tx_data.amount = self.amount.to_string();
        tx_data.vendor_field = self.vendor_field.to_owned();

        tx_data
    }
}
