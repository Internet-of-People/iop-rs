pub mod coeus;
pub mod hyd_core;
pub mod morpheus;

use super::*;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TransactionType {
    Core(hyd_core::HydraTransactionType),
    IoP(IopTransactionType),
}

impl TransactionType {
    pub fn type_group(self) -> u32 {
        match self {
            Self::Core(_) => hyd_core::HydraTransactionType::TYPE_GROUP,
            Self::IoP(_) => IopTransactionType::TYPE_GROUP,
        }
    }

    pub fn into_u16(self) -> u16 {
        match self {
            Self::Core(core_type) => core_type as u16,
            Self::IoP(iop_type) => iop_type as u16,
        }
    }
}

// TODO consider using a better programming construction than this Default here
impl Default for TransactionType {
    fn default() -> Self {
        Self::Core(hyd_core::HydraTransactionType::Transfer)
    }
}

#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[repr(u16)]
pub enum IopTransactionType {
    Morpheus = 1,
    Coeus = 2,
}

// impl Default for IoPTransactionType {
//     fn default() -> Self {
//         Self::Morpheus
//     }
// }

impl IopTransactionType {
    pub const TYPE_GROUP: u32 = 4242;

    pub fn to_bytes(&self, asset: &Asset) -> Result<Vec<u8>> {
        match asset {
            Asset::Coeus(coeus_asset) => {
                ensure!(*self == IopTransactionType::Coeus, "Expected Coeus transaction type");
                coeus_asset.to_bytes()
            }
            Asset::Morpheus(morpheus_asset) => {
                ensure!(
                    *self == IopTransactionType::Morpheus,
                    "Expected Morpheus transaction type"
                );
                morpheus_asset.to_bytes()
            }
            Asset::Core(_) => bail!("Expected IoP transaction type"),
        }
    }

    pub fn string_to_protobuf(value: &str) -> Result<Vec<u8>> {
        let mut res_bytes = Vec::new();

        let size_varint_bytes = vec![0u8; 0];
        let mut cur = Cursor::new(size_varint_bytes);
        cur.write_unsigned_varint_32(value.len() as u32)?; // NOTE: string length is size in bytes
        let size_varint_bytes = cur.into_inner();

        res_bytes.write_all(&size_varint_bytes)?;
        res_bytes.write_all(value.as_bytes())?;
        Ok(res_bytes)
    }

    pub fn protobuf_to_string(bytes: &[u8]) -> Result<String> {
        // TODO normally we should not clone the byte slice, the cursor should just read it
        //      without ownership, but reading varints is implemented only for Cursor<Vec<u8>>
        let mut cur = Cursor::new(bytes.to_owned());
        let str_length = cur.read_unsigned_varint_32()?;

        let mut str_bytes = Vec::new();
        str_bytes.resize(str_length as usize, 0u8);
        cur.read_exact(str_bytes.as_mut_slice())?;

        Ok(String::from_utf8(str_bytes)?)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Asset {
    Core(hyd_core::HydraAsset),
    Coeus(coeus::CoeusAsset),
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
