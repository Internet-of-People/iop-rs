//! DID Document types following W3C DID Core 1.0.
//!
//! DID Documents are reconstructed by replaying all operations from genesis
//! to the requested block height.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::did::{Did, DidKind, KeyId};
use crate::operation::{KeyPurpose, KeyType, Right};

/// A verification method in the DID Document.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "substrate", derive(parity_scale_codec::Encode, parity_scale_codec::Decode, scale_info::TypeInfo))]
pub struct VerificationMethod {
    /// Key identifier (fragment), e.g. "key-1".
    pub id: KeyId,
    /// Algorithm type.
    pub key_type: KeyType,
    /// Controller DID (usually the DID itself).
    pub controller: Did,
    /// Raw public key bytes.
    pub public_key: Vec<u8>,
    /// W3C verification relationships this key participates in.
    pub purposes: Vec<KeyPurpose>,
    /// Block height at which this key was added.
    pub added_at_height: u32,
    /// Optional block height at which this key expires.
    pub expires_at_height: Option<u32>,
    /// Block height at which this key was revoked (None if still active).
    pub revoked_at_height: Option<u32>,
}

impl VerificationMethod {
    /// Returns whether this key is valid (not expired, not revoked) at the given height.
    pub fn is_valid_at(&self, height: u32) -> bool {
        if height < self.added_at_height {
            return false;
        }
        if let Some(expires) = self.expires_at_height {
            if height >= expires {
                return false;
            }
        }
        if let Some(revoked) = self.revoked_at_height {
            if height >= revoked {
                return false;
            }
        }
        true
    }
}

/// A right grant record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "substrate", derive(parity_scale_codec::Encode, parity_scale_codec::Decode, scale_info::TypeInfo))]
pub struct RightGrant {
    /// The key this right is granted to.
    pub key_id: KeyId,
    /// The right granted.
    pub right: Right,
    /// Block height at which the right was granted.
    pub granted_at_height: u32,
    /// Optional block height at which the right grant expires.
    pub expires_at_height: Option<u32>,
    /// Block height at which this right was revoked (None if still active).
    pub revoked_at_height: Option<u32>,
}

impl RightGrant {
    /// Returns whether this right grant is active at the given height.
    pub fn is_active_at(&self, height: u32) -> bool {
        if height < self.granted_at_height {
            return false;
        }
        if let Some(expires) = self.expires_at_height {
            if height >= expires {
                return false;
            }
        }
        if let Some(revoked) = self.revoked_at_height {
            if height >= revoked {
                return false;
            }
        }
        true
    }
}

/// A service endpoint in the DID Document (Phase 2).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "substrate", derive(parity_scale_codec::Encode, parity_scale_codec::Decode, scale_info::TypeInfo))]
pub struct ServiceEndpoint {
    /// Service identifier fragment, e.g. "service-eurosign".
    pub id: String,
    /// Service type, e.g. "EuroSignEndpoint".
    pub service_type: String,
    /// Service endpoint URL.
    pub service_endpoint: String,
}

/// The reconstructed state of a DID Document at a specific block height.
///
/// This is the internal representation used for state management.
/// It is converted to W3C-compliant JSON for external consumption.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "substrate", derive(parity_scale_codec::Encode, parity_scale_codec::Decode, scale_info::TypeInfo))]
pub struct DidDocumentState {
    /// The DID this document describes.
    pub did: Did,
    /// Optional DID kind (persona, device, group, resource).
    pub kind: Option<DidKind>,
    /// All verification methods (keys), including revoked/expired ones for history.
    pub keys: Vec<VerificationMethod>,
    /// All right grants, including revoked ones for history.
    pub rights: Vec<RightGrant>,
    /// Service endpoints (Phase 2).
    pub services: Vec<ServiceEndpoint>,
    /// Block height at which this DID was tombstoned, if ever.
    pub tombstoned_at_height: Option<u32>,
    /// Per-key nonce for replay protection: (key_id bytes) -> nonce.
    pub nonces: BTreeMap<Vec<u8>, u64>,
}

impl DidDocumentState {
    /// Create a new empty DID document state.
    pub fn new(did: Did) -> Self {
        Self {
            did,
            kind: None,
            keys: Vec::new(),
            rights: Vec::new(),
            services: Vec::new(),
            tombstoned_at_height: None,
            nonces: BTreeMap::new(),
        }
    }

    /// Returns whether this DID is tombstoned at the given height.
    pub fn is_tombstoned_at(&self, height: u32) -> bool {
        self.tombstoned_at_height
            .map(|h| height >= h)
            .unwrap_or(false)
    }

    /// Returns whether this DID has ever been created (has at least one key).
    pub fn exists(&self) -> bool {
        !self.keys.is_empty()
    }

    /// Find a key by its KeyId.
    pub fn find_key(&self, key_id: &KeyId) -> Option<&VerificationMethod> {
        self.keys.iter().find(|k| k.id == *key_id)
    }

    /// Find a key by its KeyId (mutable).
    pub fn find_key_mut(&mut self, key_id: &KeyId) -> Option<&mut VerificationMethod> {
        self.keys.iter_mut().find(|k| k.id == *key_id)
    }

    /// Returns all currently valid keys at the given height.
    pub fn valid_keys_at(&self, height: u32) -> Vec<&VerificationMethod> {
        self.keys.iter().filter(|k| k.is_valid_at(height)).collect()
    }

    /// Check if a key has a specific right at the given height.
    pub fn has_right_at(&self, key_id: &KeyId, right: Right, height: u32) -> bool {
        // The initial key (first key added) implicitly has Update and Impersonate rights
        if let Some(first_key) = self.keys.first() {
            if first_key.id == *key_id && first_key.is_valid_at(height) {
                match right {
                    Right::Update | Right::Impersonate => return true,
                    _ => {}
                }
            }
        }

        self.rights.iter().any(|rg| {
            rg.key_id == *key_id && rg.right == right && rg.is_active_at(height)
        })
    }

    /// Get the current nonce for a key.
    pub fn nonce_for_key(&self, key_id: &KeyId) -> u64 {
        self.nonces.get(key_id.as_bytes()).copied().unwrap_or(0)
    }

    /// Increment the nonce for a key and return the new value.
    pub fn increment_nonce(&mut self, key_id: &KeyId) -> u64 {
        let entry = self.nonces.entry(key_id.as_bytes().to_vec()).or_insert(0);
        *entry += 1;
        *entry
    }

    /// Produce a W3C-compliant DID Document JSON structure at the given height.
    pub fn to_did_document_at(&self, height: u32) -> DidDocument {
        let deactivated = self.is_tombstoned_at(height);
        let valid_keys = self.valid_keys_at(height);

        let verification_methods: Vec<DidDocumentVerificationMethod> = valid_keys
            .iter()
            .map(|k| DidDocumentVerificationMethod {
                id: alloc::format!("{}#{}", self.did.to_did_string(), k.id),
                type_: key_type_to_w3c_string(k.key_type),
                controller: self.did.to_did_string(),
                public_key_multibase: multibase_encode_key(&k.public_key),
            })
            .collect();

        let authentication: Vec<String> = valid_keys
            .iter()
            .filter(|k| k.purposes.contains(&KeyPurpose::Authentication))
            .map(|k| alloc::format!("{}#{}", self.did.to_did_string(), k.id))
            .collect();

        let assertion_method: Vec<String> = valid_keys
            .iter()
            .filter(|k| k.purposes.contains(&KeyPurpose::AssertionMethod))
            .map(|k| alloc::format!("{}#{}", self.did.to_did_string(), k.id))
            .collect();

        let key_agreement: Vec<String> = valid_keys
            .iter()
            .filter(|k| k.purposes.contains(&KeyPurpose::KeyAgreement))
            .map(|k| alloc::format!("{}#{}", self.did.to_did_string(), k.id))
            .collect();

        let capability_invocation: Vec<String> = valid_keys
            .iter()
            .filter(|k| k.purposes.contains(&KeyPurpose::CapabilityInvocation))
            .map(|k| alloc::format!("{}#{}", self.did.to_did_string(), k.id))
            .collect();

        let capability_delegation: Vec<String> = valid_keys
            .iter()
            .filter(|k| k.purposes.contains(&KeyPurpose::CapabilityDelegation))
            .map(|k| alloc::format!("{}#{}", self.did.to_did_string(), k.id))
            .collect();

        DidDocument {
            context: alloc::vec![
                "https://www.w3.org/ns/did/v1".into(),
                "https://mosaic.network/ns/did/v1".into(),
            ],
            id: self.did.to_did_string(),
            controller: Some(self.did.to_did_string()),
            verification_method: verification_methods,
            authentication,
            assertion_method,
            key_agreement,
            capability_invocation,
            capability_delegation,
            service: self.services.iter().map(|s| DidDocumentService {
                id: alloc::format!("{}#{}", self.did.to_did_string(), s.id),
                type_: s.service_type.clone(),
                service_endpoint: s.service_endpoint.clone(),
            }).collect(),
            deactivated,
        }
    }
}

/// W3C DID Core 1.0 compliant DID Document for external consumption.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DidDocument {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<String>,
    #[serde(rename = "verificationMethod")]
    pub verification_method: Vec<DidDocumentVerificationMethod>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub authentication: Vec<String>,
    #[serde(rename = "assertionMethod", skip_serializing_if = "Vec::is_empty")]
    pub assertion_method: Vec<String>,
    #[serde(rename = "keyAgreement", skip_serializing_if = "Vec::is_empty")]
    pub key_agreement: Vec<String>,
    #[serde(rename = "capabilityInvocation", skip_serializing_if = "Vec::is_empty")]
    pub capability_invocation: Vec<String>,
    #[serde(rename = "capabilityDelegation", skip_serializing_if = "Vec::is_empty")]
    pub capability_delegation: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub service: Vec<DidDocumentService>,
    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub deactivated: bool,
}

/// A verification method entry in the W3C DID Document.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DidDocumentVerificationMethod {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub controller: String,
    #[serde(rename = "publicKeyMultibase")]
    pub public_key_multibase: String,
}

/// A service endpoint in the W3C DID Document.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DidDocumentService {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: String,
}

/// Convert a KeyType to its W3C verification method type string.
fn key_type_to_w3c_string(key_type: KeyType) -> String {
    match key_type {
        KeyType::Ed25519 => "Ed25519VerificationKey2020".into(),
        KeyType::Secp256k1 => "EcdsaSecp256k1VerificationKey2019".into(),
        KeyType::Secp256r1 => "EcdsaSecp256r1VerificationKey2019".into(),
        KeyType::X25519 => "X25519KeyAgreementKey2020".into(),
    }
}

/// Encode a public key as multibase base58btc.
fn multibase_encode_key(key_bytes: &[u8]) -> String {
    let encoded = bs58::encode(key_bytes).into_string();
    alloc::format!("z{}", encoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_state() -> DidDocumentState {
        let did = Did::from_public_key(b"test-pk-32-bytes-for-ed25519!!!");
        let mut state = DidDocumentState::new(did);

        // Add initial key
        state.keys.push(VerificationMethod {
            id: KeyId::from_str_id("key-1").unwrap(),
            key_type: KeyType::Ed25519,
            controller: did,
            public_key: b"test-pk-32-bytes-for-ed25519!!!".to_vec(),
            purposes: vec![KeyPurpose::Authentication, KeyPurpose::AssertionMethod],
            added_at_height: 1,
            expires_at_height: None,
            revoked_at_height: None,
        });

        state
    }

    #[test]
    fn new_state_is_empty() {
        let did = Did::default();
        let state = DidDocumentState::new(did);
        assert!(!state.exists());
        assert!(!state.is_tombstoned_at(0));
        assert!(!state.is_tombstoned_at(1000));
    }

    #[test]
    fn state_with_key_exists() {
        let state = make_test_state();
        assert!(state.exists());
    }

    #[test]
    fn key_validity_at_height() {
        let vm = VerificationMethod {
            id: KeyId::from_str_id("key-1").unwrap(),
            key_type: KeyType::Ed25519,
            controller: Did::default(),
            public_key: vec![0u8; 32],
            purposes: vec![],
            added_at_height: 10,
            expires_at_height: Some(100),
            revoked_at_height: None,
        };

        assert!(!vm.is_valid_at(9));
        assert!(vm.is_valid_at(10));
        assert!(vm.is_valid_at(50));
        assert!(vm.is_valid_at(99));
        assert!(!vm.is_valid_at(100));
    }

    #[test]
    fn key_revoked_at_height() {
        let vm = VerificationMethod {
            id: KeyId::from_str_id("key-1").unwrap(),
            key_type: KeyType::Ed25519,
            controller: Did::default(),
            public_key: vec![0u8; 32],
            purposes: vec![],
            added_at_height: 10,
            expires_at_height: None,
            revoked_at_height: Some(50),
        };

        assert!(vm.is_valid_at(10));
        assert!(vm.is_valid_at(49));
        assert!(!vm.is_valid_at(50));
        assert!(!vm.is_valid_at(100));
    }

    #[test]
    fn tombstoned_did() {
        let mut state = make_test_state();
        state.tombstoned_at_height = Some(50);

        assert!(!state.is_tombstoned_at(49));
        assert!(state.is_tombstoned_at(50));
        assert!(state.is_tombstoned_at(100));
    }

    #[test]
    fn initial_key_has_implicit_rights() {
        let state = make_test_state();
        let key_id = KeyId::from_str_id("key-1").unwrap();

        assert!(state.has_right_at(&key_id, Right::Update, 1));
        assert!(state.has_right_at(&key_id, Right::Impersonate, 1));
        assert!(!state.has_right_at(&key_id, Right::Delegate, 1));
        assert!(!state.has_right_at(&key_id, Right::Issue, 1));
    }

    #[test]
    fn explicit_right_grant() {
        let mut state = make_test_state();
        let key_id = KeyId::from_str_id("key-1").unwrap();

        state.rights.push(RightGrant {
            key_id: key_id.clone(),
            right: Right::Delegate,
            granted_at_height: 10,
            expires_at_height: Some(100),
            revoked_at_height: None,
        });

        assert!(!state.has_right_at(&key_id, Right::Delegate, 9));
        assert!(state.has_right_at(&key_id, Right::Delegate, 10));
        assert!(state.has_right_at(&key_id, Right::Delegate, 99));
        assert!(!state.has_right_at(&key_id, Right::Delegate, 100));
    }

    #[test]
    fn nonce_management() {
        let mut state = make_test_state();
        let key_id = KeyId::from_str_id("key-1").unwrap();

        assert_eq!(state.nonce_for_key(&key_id), 0);
        assert_eq!(state.increment_nonce(&key_id), 1);
        assert_eq!(state.nonce_for_key(&key_id), 1);
        assert_eq!(state.increment_nonce(&key_id), 2);
    }

    #[test]
    fn to_w3c_did_document() {
        let state = make_test_state();
        let doc = state.to_did_document_at(1);

        assert!(doc.id.starts_with("did:mosaic:z"));
        assert_eq!(doc.context.len(), 2);
        assert_eq!(doc.verification_method.len(), 1);
        assert_eq!(doc.verification_method[0].type_, "Ed25519VerificationKey2020");
        assert_eq!(doc.authentication.len(), 1);
        assert_eq!(doc.assertion_method.len(), 1);
        assert!(!doc.deactivated);
    }

    #[test]
    fn tombstoned_did_document() {
        let mut state = make_test_state();
        state.tombstoned_at_height = Some(50);

        let doc = state.to_did_document_at(100);
        assert!(doc.deactivated);
    }
}
