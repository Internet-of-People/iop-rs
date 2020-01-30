use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::data::auth::Authentication;
use crate::data::{did::Did, serde_string};
use crate::io::dist::did::ValidationStatus;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub enum Right {
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "impersonate")]
    Impersonation,
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

    pub fn has_right(
        &self, auth: &Authentication, right: Right, _after_height: BlockHeight,
        _before_height: BlockHeight,
    ) -> ValidationStatus {
        // TODO if we'll have a history in the document, this whole logic changes
        if self.tombstoned {
            return ValidationStatus::Invalid;
        }
        let keys_with_right = match self.rights.get(&right) {
            Some(key) => key,
            None => return ValidationStatus::Invalid,
        };
        for key_idx in keys_with_right.iter() {
            let key = &self.keys[*key_idx];
            if key.authentication == *auth && !key.revoked {
                // TODO check block heights against optional values present in KeyData
                return ValidationStatus::MaybeValid;
            }
        }
        ValidationStatus::Invalid
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
