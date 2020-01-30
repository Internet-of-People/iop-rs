use std::collections::HashMap;
use std::convert::TryFrom;
use std::string::ToString;

use failure::{err_msg, Fallible};
use serde::{Deserialize, Serialize};

use crate::data::{did::Did, serde_string};
use keyvault::multicipher;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub enum Right {
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "impersonate")]
    Impersonation,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(try_from = "MAuthentication", into = "MAuthentication")]
pub enum Authentication {
    KeyId(multicipher::MKeyId),
    PublicKey(multicipher::MPublicKey),
}

#[derive(Deserialize, Serialize)]
#[serde(transparent)]
struct MAuthentication(String);

impl TryFrom<MAuthentication> for Authentication {
    type Error = failure::Error;

    fn try_from(value: MAuthentication) -> Fallible<Self> {
        if value.0.starts_with(multicipher::MKeyId::PREFIX) {
            let key_id = value.0.parse()?;
            Ok(Authentication::KeyId(key_id))
        } else if value.0.starts_with(multicipher::MPublicKey::PREFIX) {
            let pk = value.0.parse()?;
            Ok(Authentication::PublicKey(pk))
        } else {
            Err(err_msg(format!("Authentication starts with invalid character: {}", value.0)))
        }
    }
}

impl Into<MAuthentication> for Authentication {
    fn into(self) -> MAuthentication {
        let string = match self {
            Authentication::KeyId(key_id) => key_id.to_string(),
            Authentication::PublicKey(pk) => pk.to_string(),
        };
        MAuthentication(string)
    }
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
    #[serde(rename = "auth")]
    pub(crate) authentication: Authentication,
    #[serde(rename = "validFromHeight")]
    pub(crate) valid_from_block: Option<BlockHeight>, // TODO should be timestamp on the long term
    #[serde(rename = "validUntilHeight")]
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
    #[serde(with = "serde_string")]
    pub(crate) did: Did,
    pub(crate) keys: Vec<KeyData>,
    // TODO should this be Vec<KeyRightPair> instead?
    pub(crate) rights: HashMap<Right, Vec<usize>>, // right -> key_indices
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) services: Vec<Service>,
    #[serde(rename = "atHeight")]
    pub(crate) at_height: usize,
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
            at_height: Default::default(),
            tombstoned: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pretty_json() {
        parse_did_document(
            r#"{
            "did": "did:morpheus:ezbeWGSY2dqcUBqT8K7R14xr",
            "keys": [
              {
                "auth": "iezbeWGSY2dqcUBqT8K7R14xr",
                "revoked": false,
                "valid": true
              },
              {
                "auth": "iez25N5WZ1Q6TQpgpyYgiu9gTX",
                "revoked": false,
                "valid": true,
                "validFromHeight": 120
              }
            ],
            "rights": {
              "impersonate": [
                0,
                1
              ],
              "update": [
                0
              ]
            },
            "atHeight": 126,
            "tombstoned": false
          }"#,
        );
    }

    #[test]
    fn terse_json() {
        parse_did_document(
            r#"{"did":"did:morpheus:ezbeWGSY2dqcUBqT8K7R14xr","keys":[{"auth":"iezbeWGSY2dqcUBqT8K7R14xr","revoked":false,"valid":true},{"auth":"iez25N5WZ1Q6TQpgpyYgiu9gTX","revoked":false,"valid":true,"validFromHeight":120}],"rights":{"impersonate":[0,1],"update":[0]},"atHeight":126,"tombstoned":false}"#,
        );
    }

    fn parse_did_document(s: &str) {
        let doc: DidDocument = serde_json::from_str(s).unwrap();

        assert_eq!(doc.did, "did:morpheus:ezbeWGSY2dqcUBqT8K7R14xr".parse().unwrap());
        assert_eq!(doc.at_height, 126);
        assert_eq!(doc.tombstoned, false);
    }
}
