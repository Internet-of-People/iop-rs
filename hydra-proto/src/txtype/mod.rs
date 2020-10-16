pub mod coeus;
pub mod hyd_core;
pub mod morpheus;

use super::*;
use crate::txtype::coeus::CoeusAsset;
use crate::txtype::hyd_core::{CoreAsset, CoreTransactionType};
use crate::txtype::morpheus::MorpheusAsset;

// #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
// pub enum TypedAsset {
//     Core(CoreTypedAsset),
//     Iop(IopTypedAsset),
// }

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
pub struct TypedAsset {
    #[serde(rename = "typeGroup")]
    pub(crate) type_group: TxTypeGroup,
    #[serde(rename = "type")]
    pub(crate) transaction_type: u16,
    #[serde(skip_serializing_if = "Asset::is_default")]
    pub(crate) asset: Asset,
}

impl Default for TypedAsset {
    fn default() -> Self {
        Self {
            type_group: TxTypeGroup::default(),
            transaction_type: CoreTransactionType::default() as u16,
            asset: Asset::default(),
        }
    }
}

impl From<(CoreTransactionType, CoreAsset)> for TypedAsset {
    fn from(value: (CoreTransactionType, CoreAsset)) -> Self {
        Self {
            type_group: TxTypeGroup::Core,
            transaction_type: value.0 as u16,
            asset: Asset::Core(value.1),
        }
    }
}

impl From<MorpheusAsset> for TypedAsset {
    fn from(value: MorpheusAsset) -> Self {
        Self {
            type_group: TxTypeGroup::Iop,
            transaction_type: IopTransactionType::Morpheus as u16,
            asset: Asset::Iop(IopAsset::Morpheus(value)),
        }
    }
}

impl From<CoeusAsset> for TypedAsset {
    fn from(value: CoeusAsset) -> Self {
        Self {
            type_group: TxTypeGroup::Iop,
            transaction_type: IopTransactionType::Coeus as u16,
            asset: Asset::Iop(IopAsset::Coeus(value)),
        }
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

impl<'de> Deserialize<'de> for TypedAsset {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct TypedAssetVisitor;

        impl<'de> SerdeVisitor<'de> for TypedAssetVisitor {
            type Value = TypedAsset;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct TypedAsset")
            }

            fn visit_map<M>(self, mut map: M) -> Result<TypedAsset, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut type_group = None;
                let mut transaction_type = None;
                let mut asset = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "typeGroup" => {
                            if type_group.is_some() {
                                return Err(de::Error::duplicate_field("typeGroup"));
                            }
                            type_group = Some(map.next_value()?);
                        }
                        "type" => {
                            if transaction_type.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            transaction_type = Some(map.next_value()?);
                        }
                        "asset" => {
                            if asset.is_some() {
                                return Err(de::Error::duplicate_field("asset"));
                            }
                            if type_group.is_none() || transaction_type.is_none() {
                                return Err(de::Error::missing_field(
                                    "typeGroup or type is missing BEFORE asset",
                                ));
                            }
                            let type_group = type_group.unwrap();
                            let transaction_type = transaction_type.unwrap();
                            match (type_group, transaction_type) {
                                (TxTypeGroup::Core, _) => {
                                    let core_asset: CoreAsset = map.next_value()?;
                                    asset = Some(Asset::Core(core_asset));
                                }
                                (TxTypeGroup::Iop, 1) => {
                                    // TODO IopTransactionType::Morpheus
                                    let morpheus_asset: MorpheusAsset = map.next_value()?;
                                    asset = Some(Asset::Iop(IopAsset::Morpheus(morpheus_asset)));
                                }
                                (TxTypeGroup::Iop, 2) => {
                                    // TODO IopTransactionType::Coeus
                                    let coeus_asset: CoeusAsset = map.next_value()?;
                                    asset = Some(Asset::Iop(IopAsset::Coeus(coeus_asset)));
                                }
                                _ => {
                                    return Err(de::Error::custom(format!(
                                        "Invalid (typeGroup,type) pair: ({:?},{})",
                                        type_group, transaction_type
                                    )))
                                }
                            }
                        }
                        other => {
                            return Err(de::Error::custom(format!("Key {} is not known.", other)));
                        }
                    }
                }

                let type_group = type_group.ok_or_else(|| de::Error::missing_field("typeGroup"))?;
                let transaction_type =
                    transaction_type.ok_or_else(|| de::Error::missing_field("type"))?;
                let asset = asset.unwrap_or(Asset::default());
                Ok(TypedAsset { type_group, transaction_type, asset })
            }
        }

        const FIELDS: &[&str] = &["typeGroup", "type", "asset"];
        deserializer.deserialize_struct("TypedAsset", FIELDS, TypedAssetVisitor)
    }
}

#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[repr(u16)]
pub enum IopTransactionType {
    Morpheus = 1,
    Coeus = 2,
}

impl IopTransactionType {
    pub const TYPE_GROUP: u32 = 4242;
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum IopAsset {
    Coeus(coeus::CoeusAsset),
    Morpheus(morpheus::MorpheusAsset),
}

impl IopAsset {
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        match self {
            IopAsset::Coeus(coeus_asset) => coeus_asset.to_bytes(),
            IopAsset::Morpheus(morpheus_asset) => morpheus_asset.to_bytes(),
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
