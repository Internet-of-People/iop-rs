use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use failure::{bail, ensure, format_err, Fallible};
use serde::{Deserialize, Serialize};

use crate::data::auth::Authentication;
use crate::data::{
    did::Did,
    serde_string,
    validation::{ValidationIssueSeverity as Severity, ValidationResult},
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub enum Right {
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "impersonate")]
    Impersonation,
}

impl Display for Right {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value =
            serde_json::to_value(self).expect("Implementation error: Right is not serializable");
        match value {
            serde_json::Value::String(s) => write!(f, "{}", s),
            _ => panic!("Implementation error: unexpected Right serialization"),
        }
    }
}

impl FromStr for Right {
    type Err = failure::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_value(serde_json::Value::String(s.to_owned()))?)
    }
}

pub type BlockHeight = usize;

pub fn is_in_opt_range(
    height: BlockHeight, from_inc: Option<BlockHeight>, until_exc: Option<BlockHeight>,
) -> bool {
    if let Some(from) = from_inc {
        if height < from {
            return false;
        }
    }
    if let Some(until) = until_exc {
        if until <= height {
            return false;
        }
    }
    true
}

pub fn is_in_range(height: BlockHeight, from_inc: BlockHeight, until_exc: BlockHeight) -> bool {
    is_in_opt_range(height, Some(from_inc), Some(until_exc))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct KeyData {
    #[serde(rename = "auth")]
    pub(crate) authentication: Authentication,
    #[serde(rename = "validFromHeight")]
    pub(crate) valid_from_block: Option<BlockHeight>, // TODO should be timestamp on the long term
    #[serde(rename = "validUntilHeight")]
    pub(crate) valid_until_block: Option<BlockHeight>, // TODO should be timestamp on the long term
    pub(crate) valid: bool,
}

impl KeyData {
    fn from_auth(authentication: Authentication) -> Self {
        Self { authentication, valid_from_block: None, valid_until_block: None, valid: true }
    }

    fn is_valid_at(&self, height: BlockHeight) -> bool {
        if let Some(valid_from) = self.valid_from_block {
            if height <= valid_from {
                return false;
            }
        }
        if let Some(valid_until) = self.valid_until_block {
            if valid_until <= height {
                return false;
            }
        }
        return true;
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub struct KeyRightHistoryItem {
    pub(crate) height: Option<usize>,
    pub(crate) valid: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub struct KeyRightHistory {
    #[serde(rename = "keyLink")]
    pub(crate) key_link: String, // TODO should be more strictly typed
    pub(crate) history: Vec<KeyRightHistoryItem>,
    pub(crate) valid: bool,
}

impl KeyRightHistory {
    fn ensure_valid_history(&self) -> Fallible<()> {
        let heights: Vec<_> =
            self.history.iter().map(|item| item.height.unwrap_or_default()).collect();
        let mut sorted = heights.clone();
        sorted.sort();
        ensure!(heights == sorted, "Height of key history items must be strictly increasing");
        Ok(())
    }

    fn is_true_at(&self, height: BlockHeight) -> Fallible<bool> {
        self.ensure_valid_history()?;

        let last_state_before_height =
            self.history.iter().rev().find(|item| item.height.unwrap_or_default() <= height);
        let valid = match last_state_before_height {
            None => false,
            Some(item) => item.valid,
        };
        Ok(valid)
    }
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
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub(crate) rights: HashMap<Right, Vec<KeyRightHistory>>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub(crate) services: Vec<Service>,
    #[serde(rename = "tombstonedAtHeight")]
    pub(crate) tombstoned_at_height: Option<usize>,
    pub(crate) tombstoned: bool,
    #[serde(rename = "queriedAtHeight")]
    pub(crate) queried_at_height: usize,
}

impl DidDocument {
    pub fn implicit(did: &Did) -> Self {
        let default_key = KeyData::from_auth(Authentication::KeyId(did.default_key_id()));
        Self {
            did: did.to_owned(),
            keys: vec![default_key],
            rights: Default::default(),
            services: Default::default(),
            tombstoned_at_height: Default::default(),
            tombstoned: Default::default(),
            queried_at_height: Default::default(),
        }
    }

    fn key(&self, key_link: &str) -> Fallible<KeyData> {
        ensure!(key_link.starts_with('#'), "Key links for remote DIDs are not supported yet");
        let idx_str: String = key_link.chars().skip(1).collect();
        let idx: usize = idx_str.parse()?;
        let key =
            self.keys.get(idx).ok_or_else(|| format_err!("No key found for link {}", key_link))?;
        Ok(key.to_owned())
    }

    fn ensure_known_height(&self, height: BlockHeight) -> Fallible<()> {
        if self.queried_at_height < height {
            bail!("Queried future height {}, present is {}", height, self.queried_at_height);
        }
        Ok(())
    }

    pub fn has_right_at(
        &self, auth: &Authentication, right: Right, height: BlockHeight,
    ) -> Fallible<bool> {
        self.ensure_known_height(height)?;

        if let Some(tombstoned_at_height) = self.tombstoned_at_height {
            if tombstoned_at_height <= height {
                return Ok(false);
            }
        }
        let keys_with_right = match self.rights.get(&right) {
            Some(key) => key,
            None => return Ok(false),
        };

        for key_right in keys_with_right.iter() {
            let key = self.key(&key_right.key_link)?;
            if !key.is_valid_at(height) {
                continue;
            }
            if key.authentication != *auth {
                continue;
            }

            return key_right.is_true_at(height);
        }

        Ok(false)
    }

    pub fn is_tombstoned_at(&self, height: BlockHeight) -> Fallible<bool> {
        self.ensure_known_height(height)?;

        if let Some(tombstone_height) = self.tombstoned_at_height {
            return Ok(tombstone_height <= height);
        }

        Ok(false)
    }

    // TODO reconsider and thoroughly check if until should be inclusive or exclusive and if implementation matches
    pub fn validate_right_between(
        &self, auth: &Authentication, right: Right, from: BlockHeight, until: BlockHeight,
    ) -> Fallible<ValidationResult> {
        ensure!(from <= until, "Invalid block range {}-{}", from, until);
        self.ensure_known_height(until)?;

        let mut result: ValidationResult = Default::default();

        if self.is_tombstoned_at(from)? {
            result.add_issue(Severity::Error, "DID was tombstoned before given period");
        }
        if let Some(tombstone_height) = self.tombstoned_at_height {
            if is_in_range(tombstone_height, from, until) {
                result.add_issue(Severity::Warning, "DID was tombstoned during given period");
            }
        }

        let keys_with_right = match self.rights.get(&right) {
            Some(entries) => entries,
            None => {
                result
                    .add_issue(Severity::Error, "Right was never granted tp given authentication");
                return Ok(result);
            }
        };

        let key_history_opt = keys_with_right.iter().find_map(|right_entry| {
            let key_data = match self.key(&right_entry.key_link) {
                Ok(key_entry) => key_entry,
                Err(e) => {
                    // TODO ideally detected earlier during parsing and should never happen here
                    result.add_issue(Severity::Error, &e.to_string());
                    return None;
                }
            };
            if key_data.authentication != *auth {
                return None;
            }
            Some((key_data, &right_entry.history))
        });

        let (key_data, history) = match key_history_opt {
            Some(key_history) => key_history,
            None => {
                result.add_issue(Severity::Error, "No matching authentication found in DID");
                return Ok(result);
            }
        };

        if let Some(key_valid_from) = key_data.valid_from_block {
            if until < key_valid_from {
                result.add_issue(Severity::Error, "Key was enabled only after given period");
            }
            if is_in_range(key_valid_from, from, until) {
                result.add_issue(Severity::Warning, "Key was enabled during given period");
            }
        }

        if let Some(key_valid_until) = key_data.valid_until_block {
            if key_valid_until < from {
                result.add_issue(Severity::Error, "Key expired before given period");
            }
            if is_in_range(key_valid_until, from, until) {
                result.add_issue(Severity::Warning, "Key expired during given period");
            }
        }

        ensure!(! history.is_empty(), "Implementation error: key related to rights were already filtered, right must be present here");

        if history.len() == 1 {
            let change = &history[0];
            ensure!(change.valid, "History is false by default, first entry should set it to true");

            if let Some(height) = change.height {
                if is_in_range(height, from, until) {
                    result.add_issue(Severity::Warning, "Key expired during given period");
                }
                if until <= height {
                    result.add_issue(
                        Severity::Error,
                        "Key was granted specified right only after given period",
                    );
                }
            }

            return Ok(result);
        }

        // Found multiple changes in history
        let mut last_state_before_range: Option<&KeyRightHistoryItem> = None;
        for item in history {
            if is_in_range(item.height.unwrap_or_default(), from, until) {
                result.add_issue(Severity::Warning, "Required right changed during givenn period");
                ensure!(
                    item.valid
                        || (last_state_before_range.is_some()
                            && last_state_before_range.unwrap().valid),
                    "Implementation error: right history must contain alternating values"
                );
                return Ok(result);
            }
            if item.height.unwrap_or_default() <= from {
                last_state_before_range = Some(item);
            } else {
                break;
            }
        }

        let valid = match last_state_before_range {
            Some(state) => state.valid, // right explicitly denied
            None => false,              // Rights are denied by default
        };
        if !valid {
            result.add_issue(Severity::Error, "Required right is not present for key");
        }

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pretty_json() {
        test_parsed_did_document(
            r##"{
            "did": "did:morpheus:ezbeWGSY2dqcUBqT8K7R14xr",
            "keys": [
              {
                "auth": "iezbeWGSY2dqcUBqT8K7R14xr",
                "valid": true
              },
              {
                "auth": "iez25N5WZ1Q6TQpgpyYgiu9gTX",
                "valid": true,
                "validFromHeight": 120
              }
            ],
            "rights": {
              "impersonate": [
                {
                  "keyLink": "#0",
                  "history": [
                    {
                      "height": null,
                      "valid": true
                    }
                  ],
                  "valid": true
                },
                {
                  "keyLink": "#1",
                  "history": [
                    {
                      "height": null,
                      "valid": false
                    },
                    {
                      "height": 126,
                      "valid": true
                    }
                  ],
                  "valid": true
                }
              ],
              "update": [
                {
                  "keyLink": "#0",
                  "history": [
                    {
                      "height": null,
                      "valid": true
                    }
                  ],
                  "valid": true
                },
                {
                  "keyLink": "#1",
                  "history": [
                    {
                      "height": null,
                      "valid": false
                    }
                  ],
                  "valid": false
                }
              ]
            },
            "tombstonedAtHeight": null,
            "tombstoned": false,
            "queriedAtHeight": 126
          }"##,
        );
    }

    #[test]
    fn terse_json() {
        test_parsed_did_document(
            r##"{"did":"did:morpheus:ezbeWGSY2dqcUBqT8K7R14xr","keys":[{"index":0,"auth":"iezbeWGSY2dqcUBqT8K7R14xr","validFromHeight":null,"validUntilHeight":null,"valid":true},{"index":1,"auth":"iez25N5WZ1Q6TQpgpyYgiu9gTX","validFromHeight":120,"validUntilHeight":null,"valid":true}],"rights":{"impersonate":[{"keyLink":"#0","history":[{"height":null,"valid":true}],"valid":true},{"keyLink":"#1","history":[{"height":null,"valid":false},{"height":126,"valid":true}],"valid":true}],"update":[{"keyLink":"#0","history":[{"height":null,"valid":true}],"valid":true},{"keyLink":"#1","history":[{"height":null,"valid":false}],"valid":false}]},"tombstoned":false,"tombstonedAtHeight":null,"queriedAtHeight":126}"##,
        );
    }

    fn test_parsed_did_document(s: &str) {
        let doc: DidDocument = serde_json::from_str(s).unwrap();

        assert_eq!(doc.did, "did:morpheus:ezbeWGSY2dqcUBqT8K7R14xr".parse().unwrap());
        assert_eq!(doc.tombstoned_at_height, None);
        assert_eq!(doc.queried_at_height, 126);
        assert_eq!(doc.tombstoned, false);

        let first_key = &doc.keys[0].authentication;
        let second_key = &doc.keys[1].authentication;

        assert!(doc.has_right_at(first_key, Right::Impersonation, 1).unwrap());
        assert!(doc.has_right_at(first_key, Right::Impersonation, 2).unwrap());
        assert!(doc.has_right_at(first_key, Right::Impersonation, 125).unwrap());
        assert!(doc.has_right_at(first_key, Right::Impersonation, 126).unwrap());
        assert!(doc.has_right_at(first_key, Right::Impersonation, 127).is_err());

        assert!(!doc.has_right_at(second_key, Right::Impersonation, 1).unwrap());
        assert!(!doc.has_right_at(second_key, Right::Impersonation, 2).unwrap());
        assert!(!doc.has_right_at(second_key, Right::Impersonation, 125).unwrap());
        assert!(doc.has_right_at(second_key, Right::Impersonation, 126).unwrap());
        assert!(doc.has_right_at(second_key, Right::Impersonation, 127).is_err());

        assert!(doc.has_right_at(first_key, Right::Update, 1).unwrap());
        assert!(doc.has_right_at(first_key, Right::Update, 2).unwrap());
        assert!(doc.has_right_at(first_key, Right::Update, 125).unwrap());
        assert!(doc.has_right_at(first_key, Right::Update, 126).unwrap());
        assert!(doc.has_right_at(first_key, Right::Update, 127).is_err());

        assert!(!doc.has_right_at(second_key, Right::Update, 1).unwrap());
        assert!(!doc.has_right_at(second_key, Right::Update, 2).unwrap());
        assert!(!doc.has_right_at(second_key, Right::Update, 125).unwrap());
        assert!(!doc.has_right_at(second_key, Right::Update, 126).unwrap());
        assert!(doc.has_right_at(second_key, Right::Update, 127).is_err());
    }

    #[test]
    fn has_right_between() {}
}
