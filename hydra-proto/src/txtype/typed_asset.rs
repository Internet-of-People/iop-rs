use super::*;

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
                let asset = asset.unwrap_or_default();
                Ok(TypedAsset { type_group, transaction_type, asset })
            }
        }

        const FIELDS: &[&str] = &["typeGroup", "type", "asset"];
        deserializer.deserialize_struct("TypedAsset", FIELDS, TypedAssetVisitor)
    }
}
