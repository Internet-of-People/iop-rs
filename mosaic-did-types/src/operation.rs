//! DID operations for the `did:mosaic` method.
//!
//! All SSI state changes flow through signed operations submitted in atomic batches.
//! Phase 1 (MVP) implements the 5 core DID operations + BeforeProof.

use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::did::{ContentId, Did, KeyId};
use crate::signature::MultiSignature;

/// Rights that can be assigned to keys in a DID Document.
///
/// The rights model extends IOP Morpheus (Update, Impersonate) with
/// additional capabilities for enterprise and compliance use cases.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "substrate", derive(parity_scale_codec::Encode, parity_scale_codec::Decode, parity_scale_codec::MaxEncodedLen, scale_info::TypeInfo))]
pub enum Right {
    /// Can modify the DID Document (add/revoke keys, rights).
    Update,
    /// Can act on behalf of the DID (sign as the DID).
    Impersonate,
    /// Can grant rights to other keys.
    Delegate,
    /// Can issue Verifiable Credentials from this DID.
    Issue,
    /// Can revoke Verifiable Credentials from this DID.
    Revoke,
    /// Can perform RecoverDid operation (Phase 2).
    Recovery,
}

impl Right {
    /// Returns all Phase 1 (MVP) rights.
    pub fn phase1_variants() -> &'static [Right] {
        &[
            Right::Update,
            Right::Impersonate,
            Right::Delegate,
            Right::Issue,
            Right::Revoke,
            Right::Recovery,
        ]
    }
}

impl core::fmt::Display for Right {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Right::Update => write!(f, "update"),
            Right::Impersonate => write!(f, "impersonate"),
            Right::Delegate => write!(f, "delegate"),
            Right::Issue => write!(f, "issue"),
            Right::Revoke => write!(f, "revoke"),
            Right::Recovery => write!(f, "recovery"),
        }
    }
}

/// Supported key types for verification methods.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "substrate", derive(parity_scale_codec::Encode, parity_scale_codec::Decode, parity_scale_codec::MaxEncodedLen, scale_info::TypeInfo))]
pub enum KeyType {
    /// Ed25519 — default, required.
    Ed25519,
    /// secp256k1 — Ethereum compatibility.
    Secp256k1,
    /// secp256r1 (P-256) — HSM compatibility (Phase 2).
    Secp256r1,
    /// X25519 — key agreement only (Phase 2).
    X25519,
}

impl Default for KeyType {
    fn default() -> Self {
        KeyType::Ed25519
    }
}

/// Key purposes per W3C DID Core verification relationships.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "substrate", derive(parity_scale_codec::Encode, parity_scale_codec::Decode, parity_scale_codec::MaxEncodedLen, scale_info::TypeInfo))]
pub enum KeyPurpose {
    /// Prove control of DID (login, challenge-response).
    Authentication,
    /// Sign Verifiable Credentials.
    AssertionMethod,
    /// Establish encrypted communication channel.
    KeyAgreement,
    /// Invoke capabilities.
    CapabilityInvocation,
    /// Delegate capabilities.
    CapabilityDelegation,
}

/// Phase 1 (MVP) DID operations.
///
/// All operations are submitted inside `SignedOperation` bundles within an
/// atomic `submit_did_operations` batch.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "substrate", derive(parity_scale_codec::Encode, parity_scale_codec::Decode, scale_info::TypeInfo))]
pub enum DidOperation {
    /// Add a new key to the DID document.
    ///
    /// Implicit creation: the first `AddKey` for a DID creates that DID.
    /// The DID identifier MUST equal `BLAKE3-256(public_key)` for the initial key.
    AddKey {
        did: Did,
        key_id: KeyId,
        key_type: KeyType,
        public_key: Vec<u8>,
        purposes: Vec<KeyPurpose>,
        /// Optional block height at which this key expires.
        expires_at_height: Option<u32>,
    },

    /// Revoke an existing key from the DID document.
    RevokeKey {
        did: Did,
        key_id: KeyId,
    },

    /// Grant a right to a key.
    AddRight {
        did: Did,
        key_id: KeyId,
        right: Right,
        /// Optional block height at which this right grant expires.
        expires_at_height: Option<u32>,
    },

    /// Revoke a right from a key.
    RevokeRight {
        did: Did,
        key_id: KeyId,
        right: Right,
    },

    /// Permanently deactivate the DID. Irreversible.
    ///
    /// After tombstoning:
    /// - Resolution returns `deactivated: true`
    /// - No further operations are accepted
    /// - Historical state remains queryable
    TombstoneDid {
        did: Did,
    },
}

impl DidOperation {
    /// Returns the DID this operation targets.
    pub fn did(&self) -> &Did {
        match self {
            DidOperation::AddKey { did, .. } => did,
            DidOperation::RevokeKey { did, .. } => did,
            DidOperation::AddRight { did, .. } => did,
            DidOperation::RevokeRight { did, .. } => did,
            DidOperation::TombstoneDid { did } => did,
        }
    }

    /// Returns the required Right to perform this operation.
    pub fn required_right(&self) -> Right {
        match self {
            DidOperation::AddKey { .. } => Right::Update,
            DidOperation::RevokeKey { .. } => Right::Update,
            DidOperation::AddRight { .. } => Right::Delegate,
            DidOperation::RevokeRight { .. } => Right::Delegate,
            DidOperation::TombstoneDid { .. } => Right::Update,
        }
    }
}

/// A top-level SSI operation: either a DID operation or a BeforeProof timestamp.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "substrate", derive(parity_scale_codec::Encode, parity_scale_codec::Decode, scale_info::TypeInfo))]
pub enum SsiOperation {
    /// A signed DID document operation.
    Did(SignedDidOperation),
    /// A BeforeProof timestamp registration (permissionless).
    BeforeProof(ContentId),
}

/// A signed DID operation bundle with proof of authorization.
///
/// The signature proves the signer has authority over the operation.
/// The nonce provides replay protection.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "substrate", derive(parity_scale_codec::Encode, parity_scale_codec::Decode, scale_info::TypeInfo))]
pub struct SignedDidOperation {
    /// The DID operation to execute.
    pub operation: DidOperation,
    /// The key identifier of the signer within the target DID document.
    pub signer_key_id: KeyId,
    /// Cryptographic signature over the operation.
    pub signature: MultiSignature,
    /// Monotonic nonce for replay protection (per-DID, per-key).
    pub nonce: u64,
}

/// Record stored on-chain for a registered BeforeProof.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "substrate", derive(parity_scale_codec::Encode, parity_scale_codec::Decode, parity_scale_codec::MaxEncodedLen, scale_info::TypeInfo))]
pub struct BeforeProofRecord {
    /// Block height at which the proof was registered.
    pub block_height: u32,
    /// Index of the extrinsic within the block.
    pub extrinsic_index: u32,
}

/// Type of DID operation (for events, without carrying full data).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "substrate", derive(parity_scale_codec::Encode, parity_scale_codec::Decode, scale_info::TypeInfo))]
pub enum DidOperationType {
    AddKey,
    RevokeKey,
    AddRight,
    RevokeRight,
    TombstoneDid,
}

impl From<&DidOperation> for DidOperationType {
    fn from(op: &DidOperation) -> Self {
        match op {
            DidOperation::AddKey { .. } => DidOperationType::AddKey,
            DidOperation::RevokeKey { .. } => DidOperationType::RevokeKey,
            DidOperation::AddRight { .. } => DidOperationType::AddRight,
            DidOperation::RevokeRight { .. } => DidOperationType::RevokeRight,
            DidOperation::TombstoneDid { .. } => DidOperationType::TombstoneDid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation_did_accessor() {
        let did = Did::from_public_key(b"test-key");
        let key_id = KeyId::from_str_id("key-1").unwrap();

        let op = DidOperation::AddKey {
            did,
            key_id,
            key_type: KeyType::Ed25519,
            public_key: b"test-key".to_vec(),
            purposes: vec![KeyPurpose::Authentication, KeyPurpose::AssertionMethod],
            expires_at_height: None,
        };

        assert_eq!(*op.did(), did);
        assert_eq!(op.required_right(), Right::Update);
    }

    #[test]
    fn operation_type_conversion() {
        let did = Did::default();
        let key_id = KeyId::from_str_id("key-1").unwrap();

        let ops = vec![
            DidOperation::AddKey {
                did,
                key_id: key_id.clone(),
                key_type: KeyType::Ed25519,
                public_key: vec![],
                purposes: vec![],
                expires_at_height: None,
            },
            DidOperation::RevokeKey { did, key_id: key_id.clone() },
            DidOperation::AddRight { did, key_id: key_id.clone(), right: Right::Update, expires_at_height: None },
            DidOperation::RevokeRight { did, key_id: key_id.clone(), right: Right::Update },
            DidOperation::TombstoneDid { did },
        ];

        let types: Vec<DidOperationType> = ops.iter().map(DidOperationType::from).collect();
        assert_eq!(types, vec![
            DidOperationType::AddKey,
            DidOperationType::RevokeKey,
            DidOperationType::AddRight,
            DidOperationType::RevokeRight,
            DidOperationType::TombstoneDid,
        ]);
    }

    #[test]
    fn right_display() {
        assert_eq!(Right::Update.to_string(), "update");
        assert_eq!(Right::Impersonate.to_string(), "impersonate");
        assert_eq!(Right::Delegate.to_string(), "delegate");
        assert_eq!(Right::Issue.to_string(), "issue");
        assert_eq!(Right::Revoke.to_string(), "revoke");
        assert_eq!(Right::Recovery.to_string(), "recovery");
    }
}
