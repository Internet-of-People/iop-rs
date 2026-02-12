//! On-chain DID document state types using bounded collections.
//!
//! These types are designed for direct on-chain storage in Substrate pallets,
//! using `BoundedVec` and `BoundedBTreeMap` for deterministic memory usage.

use sp_std::vec::Vec;
use frame_support::pallet_prelude::*;
use frame_support::{BoundedBTreeMap, BoundedVec};

use mosaic_did_types::{Did, DidKind, KeyId, KeyPurpose, KeyType, Right};

/// Maximum number of keys per DID document.
pub const MAX_KEYS_PER_DID: u32 = 32;
/// Maximum number of right grants per DID document.
pub const MAX_RIGHTS_PER_DID: u32 = 64;
/// Maximum number of key purposes per verification method.
pub const MAX_PURPOSES_PER_KEY: u32 = 5;
/// Maximum number of nonce entries per DID document.
pub const MAX_NONCES_PER_DID: u32 = 32;
/// Maximum public key size in bytes.
pub const MAX_PUBLIC_KEY_LEN: u32 = 64;
/// Maximum key ID size in bytes.
pub const MAX_KEY_ID_LEN: u32 = 64;

/// A verification method stored on-chain with bounded collections.
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct OnChainVerificationMethod {
    /// Key identifier (fragment), e.g. "key-1".
    pub id: BoundedVec<u8, ConstU32<MAX_KEY_ID_LEN>>,
    /// Algorithm type.
    pub key_type: KeyType,
    /// Controller DID (usually the DID itself).
    pub controller: Did,
    /// Raw public key bytes.
    pub public_key: BoundedVec<u8, ConstU32<MAX_PUBLIC_KEY_LEN>>,
    /// W3C verification relationships this key participates in.
    pub purposes: BoundedVec<KeyPurpose, ConstU32<MAX_PURPOSES_PER_KEY>>,
    /// Block height at which this key was added.
    pub added_at_height: u32,
    /// Optional block height at which this key expires.
    pub expires_at_height: Option<u32>,
    /// Block height at which this key was revoked (None if still active).
    pub revoked_at_height: Option<u32>,
}

impl OnChainVerificationMethod {
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

    /// Get the key ID as a KeyId type.
    pub fn key_id(&self) -> Option<KeyId> {
        KeyId::new(self.id.to_vec()).ok()
    }
}

/// A right grant record stored on-chain.
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct OnChainRightGrant {
    /// The key this right is granted to (raw bytes).
    pub key_id: BoundedVec<u8, ConstU32<MAX_KEY_ID_LEN>>,
    /// The right granted.
    pub right: Right,
    /// Block height at which the right was granted.
    pub granted_at_height: u32,
    /// Optional block height at which the right grant expires.
    pub expires_at_height: Option<u32>,
    /// Block height at which this right was revoked (None if still active).
    pub revoked_at_height: Option<u32>,
}

impl OnChainRightGrant {
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

/// On-chain DID document state with bounded collections.
///
/// Stored directly in `StorageMap<Did, OnChainDidState>`.
/// No event-log replay needed — state is always materialized.
#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct OnChainDidState {
    /// The DID this document describes.
    pub did: Did,
    /// Optional DID kind.
    pub kind: Option<DidKind>,
    /// All verification methods (keys), including revoked/expired ones for history.
    pub keys: BoundedVec<OnChainVerificationMethod, ConstU32<MAX_KEYS_PER_DID>>,
    /// All right grants, including revoked ones for history.
    pub rights: BoundedVec<OnChainRightGrant, ConstU32<MAX_RIGHTS_PER_DID>>,
    /// Block height at which this DID was tombstoned, if ever.
    pub tombstoned_at_height: Option<u32>,
    /// Per-key nonce for replay protection: key_id bytes -> nonce.
    pub nonces: BoundedBTreeMap<
        BoundedVec<u8, ConstU32<MAX_KEY_ID_LEN>>,
        u64,
        ConstU32<MAX_NONCES_PER_DID>,
    >,
}

impl Default for OnChainDidState {
    fn default() -> Self {
        Self {
            did: Did::default(),
            kind: None,
            keys: BoundedVec::default(),
            rights: BoundedVec::default(),
            tombstoned_at_height: None,
            nonces: BoundedBTreeMap::default(),
        }
    }
}

impl OnChainDidState {
    /// Create a new empty on-chain DID state.
    pub fn new(did: Did) -> Self {
        Self {
            did,
            ..Default::default()
        }
    }

    /// Returns whether this DID exists (has at least one key).
    pub fn exists(&self) -> bool {
        !self.keys.is_empty()
    }

    /// Returns whether this DID is tombstoned at the given height.
    pub fn is_tombstoned_at(&self, height: u32) -> bool {
        self.tombstoned_at_height
            .map(|h| height >= h)
            .unwrap_or(false)
    }

    /// Find a key by its raw key_id bytes.
    pub fn find_key(&self, key_id_bytes: &[u8]) -> Option<&OnChainVerificationMethod> {
        self.keys.iter().find(|k| k.id.as_slice() == key_id_bytes)
    }

    /// Find a key by its raw key_id bytes (mutable).
    pub fn find_key_mut(&mut self, key_id_bytes: &[u8]) -> Option<&mut OnChainVerificationMethod> {
        self.keys.iter_mut().find(|k| k.id.as_slice() == key_id_bytes)
    }

    /// Returns all currently valid keys at the given height.
    pub fn valid_keys_at(&self, height: u32) -> Vec<&OnChainVerificationMethod> {
        self.keys.iter().filter(|k| k.is_valid_at(height)).collect()
    }

    /// Check if a key has a specific right at the given height.
    pub fn has_right_at(&self, key_id_bytes: &[u8], right: Right, height: u32) -> bool {
        // The initial key (first key added) implicitly has Update and Impersonate rights
        if let Some(first_key) = self.keys.first() {
            if first_key.id.as_slice() == key_id_bytes && first_key.is_valid_at(height) {
                match right {
                    Right::Update | Right::Impersonate => return true,
                    _ => {}
                }
            }
        }

        self.rights.iter().any(|rg| {
            rg.key_id.as_slice() == key_id_bytes && rg.right == right && rg.is_active_at(height)
        })
    }

    /// Get the current nonce for a key.
    pub fn nonce_for_key(&self, key_id_bytes: &[u8]) -> u64 {
        for (k, v) in self.nonces.iter() {
            if k.as_slice() == key_id_bytes {
                return *v;
            }
        }
        0
    }
}
