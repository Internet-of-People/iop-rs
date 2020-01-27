use std::collections::HashMap;

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

pub type BlockHeight = usize;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct KeyData {
    authentication: Authentication,
    valid_from_block: Option<BlockHeight>, // TODO should be timestamp on the long term
    valid_until_block: Option<BlockHeight>, // TODO should be timestamp on the long term
    revoked: bool,
}

impl KeyData {
    fn from_auth(authentication: Authentication) -> Self {
        Self { authentication, valid_from_block: None, valid_until_block: None, revoked: false }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub struct KeyRightPair {
    right: Right,
    key_index: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub enum ServiceType {
    // TODO
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub struct Service {
    #[serde(rename = "type")]
    type_: ServiceType,
    name: String,
    service_endpoint: String, // TODO should we use multiaddr::Multiaddr here and thus add CID-dependency?
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DidDocument {
    #[serde(rename = "id")]
    did: Did,
    keys: Vec<KeyData>,
    // TODO should this be Vec<KeyRightPair> instead?
    rights: HashMap<Right, Vec<usize>>, // right -> key_indices
    services: Vec<Service>,
    tombstoned: bool,
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
