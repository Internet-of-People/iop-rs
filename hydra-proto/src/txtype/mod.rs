pub mod coeus;
pub mod hyd_core;
mod iop;
pub mod morpheus;
mod typed_asset;

pub use iop::*;
pub use typed_asset::*; // TODO Move transaction.rs and serializer.rs into this module, then remove pub from this use

use super::*;
use crate::txtype::hyd_core::{CoreAsset, CoreTransactionType};
use crate::txtype::morpheus::MorpheusAsset;

#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[repr(u32)]
pub enum TxTypeGroup {
    Core = 1,
    Iop = 4242,
}

impl Default for TxTypeGroup {
    fn default() -> Self {
        Self::Core
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Asset {
    Core(CoreAsset),
    Iop(IopAsset),
}

impl Asset {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

impl Default for Asset {
    fn default() -> Self {
        Self::Core(CoreAsset::default())
    }
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
    pub optional: OptionalTransactionFields,
}

#[derive(Clone, Debug, Default)]
pub struct OptionalTransactionFields {
    pub amount: u64,
    pub manual_fee: Option<u64>,
    pub vendor_field: Option<String>,
}

impl CommonTransactionFields {
    pub fn calculate_fee(&self, tx: &dyn Aip29Transaction) -> u64 {
        self.optional.manual_fee.unwrap_or_else(|| tx.fee())
    }

    fn to_data(&self) -> TransactionData {
        let mut tx_data = TransactionData::default();
        tx_data.network = Some(self.network.p2pkh_addr()[0]);
        tx_data.version = Some(2);

        tx_data.sender_public_key = self.sender_public_key.to_string();
        tx_data.nonce = Some(self.nonce.to_string());
        tx_data.amount = self.optional.amount.to_string();
        tx_data.vendor_field = self.optional.vendor_field.to_owned();

        tx_data
    }
}
