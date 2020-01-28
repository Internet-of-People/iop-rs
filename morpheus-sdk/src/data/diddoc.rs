use std::collections::HashMap;
use std::string::ToString;

use serde::{Deserialize, Serialize};

use crate::data::did::Did;
use keyvault::multicipher;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub enum Right {
    Update,
    Impersonation,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Authentication {
    KeyId(multicipher::MKeyId),
    PublicKey(multicipher::MPublicKey),
}

impl ToString for Authentication {
    fn to_string(&self) -> String {
        match self {
            Self::KeyId(id) => id.to_string(),
            Self::PublicKey(key) => key.to_string(),
        }
    }
}

pub type BlockHeight = usize;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct KeyData {
    pub(crate) authentication: Authentication,
    pub(crate) valid_from_block: Option<BlockHeight>, // TODO should be timestamp on the long term
    pub(crate) valid_until_block: Option<BlockHeight>, // TODO should be timestamp on the long term
    pub(crate) revoked: bool,
}

impl KeyData {
    fn from_auth(authentication: Authentication) -> Self {
        Self { authentication, valid_from_block: None, valid_until_block: None, revoked: false }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub struct KeyRightPair {
    pub(crate) right: Right,
    pub(crate) key_index: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub enum ServiceType {
    // TODO
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub struct Service {
    #[serde(rename = "type")]
    pub(crate) type_: ServiceType,
    pub(crate) name: String,
    pub(crate) service_endpoint: String, // TODO should we use multiaddr::Multiaddr here and thus add CID-dependency?
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DidDocument {
    #[serde(rename = "id")]
    pub(crate) did: Did,
    pub(crate) keys: Vec<KeyData>,
    // TODO should this be Vec<KeyRightPair> instead?
    pub(crate) rights: HashMap<Right, Vec<usize>>, // right -> key_indices
    pub(crate) services: Vec<Service>,
    pub(crate) tombstoned: bool,
}

impl DidDocument {
    pub fn implicit(did: &Did) -> Self {
        let default_key = KeyData::from_auth(Authentication::KeyId(did.default_key_id()));
        Self {
            did: did.to_owned(),
            keys: vec![default_key],
            rights: Default::default(),
            services: Default::default(),
            tombstoned: Default::default(),
        }
    }
}
