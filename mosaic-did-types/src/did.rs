//! DID identifier types for `did:mosaic`.
//!
//! A Mosaic DID is derived from the initial public key:
//! ```text
//! did:mosaic:<multibase(base58btc, BLAKE3-256(initial-public-key))>
//! ```

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

use serde::{Deserialize, Serialize};

/// A 32-byte Mosaic DID identifier, derived as `BLAKE3-256(initial_public_key)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "substrate", derive(
    parity_scale_codec::Encode, parity_scale_codec::Decode,
    parity_scale_codec::MaxEncodedLen, scale_info::TypeInfo,
))]
pub struct Did(pub [u8; 32]);

impl Did {
    /// The DID method prefix.
    pub const METHOD_PREFIX: &'static str = "did:mosaic:";

    /// The legacy IOP Morpheus prefix (for migration support).
    pub const LEGACY_MORPHEUS_PREFIX: &'static str = "did:morpheus:";

    /// Derive a DID identifier from a public key using BLAKE3-256.
    pub fn from_public_key(public_key: &[u8]) -> Self {
        let hash = blake3::hash(public_key);
        Did(*hash.as_bytes())
    }

    /// Return the raw 32-byte identifier.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Encode the DID as a full `did:mosaic:z...` string using multibase base58btc.
    pub fn to_did_string(&self) -> String {
        // Multibase base58btc prefix is 'z'
        let encoded = bs58::encode(&self.0).into_string();
        let mut s = String::with_capacity(Self::METHOD_PREFIX.len() + 1 + encoded.len());
        s.push_str(Self::METHOD_PREFIX);
        s.push('z');
        s.push_str(&encoded);
        s
    }

    /// Parse a `did:mosaic:z...` string back into a Did.
    pub fn from_did_string(s: &str) -> Result<Self, DidParseError> {
        let rest = s
            .strip_prefix(Self::METHOD_PREFIX)
            .ok_or(DidParseError::InvalidPrefix)?;

        // Expect multibase base58btc prefix 'z'
        let base58_str = rest
            .strip_prefix('z')
            .ok_or(DidParseError::InvalidMultibase)?;

        let bytes = bs58::decode(base58_str)
            .into_vec()
            .map_err(|_| DidParseError::InvalidBase58)?;

        if bytes.len() != 32 {
            return Err(DidParseError::InvalidLength(bytes.len()));
        }

        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        Ok(Did(arr))
    }
}

impl fmt::Display for Did {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_did_string())
    }
}

impl Default for Did {
    fn default() -> Self {
        Did([0u8; 32])
    }
}

impl From<[u8; 32]> for Did {
    fn from(bytes: [u8; 32]) -> Self {
        Did(bytes)
    }
}

/// Errors that can occur when parsing a DID string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DidParseError {
    /// The string doesn't start with `did:mosaic:`.
    InvalidPrefix,
    /// Missing or wrong multibase prefix (expected 'z' for base58btc).
    InvalidMultibase,
    /// Invalid base58 encoding.
    InvalidBase58,
    /// Decoded bytes are not 32 bytes long.
    InvalidLength(usize),
}

impl fmt::Display for DidParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPrefix => write!(f, "DID must start with '{}'", Did::METHOD_PREFIX),
            Self::InvalidMultibase => write!(f, "expected multibase base58btc prefix 'z'"),
            Self::InvalidBase58 => write!(f, "invalid base58 encoding"),
            Self::InvalidLength(len) => write!(f, "expected 32 bytes, got {}", len),
        }
    }
}

/// Key identifier within a DID document.
///
/// References a specific verification method, e.g. `"key-1"`, `"key-recovery"`.
/// Stored as raw bytes with a maximum length of 64.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "substrate", derive(
    parity_scale_codec::Encode, parity_scale_codec::Decode, scale_info::TypeInfo,
))]
pub struct KeyId(Vec<u8>);

impl KeyId {
    /// Maximum length of a key identifier in bytes.
    pub const MAX_LEN: usize = 64;

    /// Create a new KeyId from bytes.
    pub fn new(bytes: Vec<u8>) -> Result<Self, &'static str> {
        if bytes.len() > Self::MAX_LEN {
            return Err("KeyId exceeds maximum length of 64 bytes");
        }
        Ok(KeyId(bytes))
    }

    /// Create a KeyId from a string identifier (e.g. "key-1").
    pub fn from_str_id(s: &str) -> Result<Self, &'static str> {
        Self::new(s.as_bytes().to_vec())
    }

    /// Return the raw bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Return as a UTF-8 string if valid.
    pub fn as_str(&self) -> Option<&str> {
        core::str::from_utf8(&self.0).ok()
    }
}

impl fmt::Display for KeyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.as_str() {
            Some(s) => write!(f, "{}", s),
            None => write!(f, "0x{}", hex_encode(&self.0)),
        }
    }
}

/// A 32-byte content identifier for BeforeProof (BLAKE3 hash of content).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "substrate", derive(
    parity_scale_codec::Encode, parity_scale_codec::Decode,
    parity_scale_codec::MaxEncodedLen, scale_info::TypeInfo,
))]
pub struct ContentId(pub [u8; 32]);

impl ContentId {
    /// Compute a ContentId from arbitrary content bytes using BLAKE3.
    pub fn from_content(content: &[u8]) -> Self {
        let hash = blake3::hash(content);
        ContentId(*hash.as_bytes())
    }

    /// Return the raw 32-byte hash.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl Default for ContentId {
    fn default() -> Self {
        ContentId([0u8; 32])
    }
}

impl From<[u8; 32]> for ContentId {
    fn from(bytes: [u8; 32]) -> Self {
        ContentId(bytes)
    }
}

impl fmt::Display for ContentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex_encode(&self.0))
    }
}

/// DID kinds following IOP Morpheus architecture.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "substrate", derive(
    parity_scale_codec::Encode, parity_scale_codec::Decode,
    parity_scale_codec::MaxEncodedLen, scale_info::TypeInfo,
))]
pub enum DidKind {
    /// Human identity (end users, signers).
    Persona,
    /// Hardware/software agent (mobile apps, HSMs, IoT).
    Device,
    /// Organizational identity (companies, DAOs).
    Group,
    /// Non-agent entities (documents, invoices, data assets).
    Resource,
}

impl Default for DidKind {
    fn default() -> Self {
        DidKind::Persona
    }
}

// Simple hex encoding helper (no_std compatible)
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| alloc::format!("{:02x}", b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn did_from_public_key_deterministic() {
        let pk = b"test-public-key-ed25519-32bytes!";
        let did1 = Did::from_public_key(pk);
        let did2 = Did::from_public_key(pk);
        assert_eq!(did1, did2);
    }

    #[test]
    fn did_different_keys_different_dids() {
        let did1 = Did::from_public_key(b"key-alice");
        let did2 = Did::from_public_key(b"key-bob");
        assert_ne!(did1, did2);
    }

    #[test]
    fn did_string_roundtrip() {
        let did = Did::from_public_key(b"test-public-key-ed25519-32bytes!");
        let did_string = did.to_did_string();
        assert!(did_string.starts_with("did:mosaic:z"));

        let parsed = Did::from_did_string(&did_string).unwrap();
        assert_eq!(did, parsed);
    }

    #[test]
    fn did_parse_invalid_prefix() {
        let err = Did::from_did_string("did:morpheus:z123").unwrap_err();
        assert_eq!(err, DidParseError::InvalidPrefix);
    }

    #[test]
    fn did_parse_invalid_multibase() {
        let err = Did::from_did_string("did:mosaic:f123").unwrap_err();
        assert_eq!(err, DidParseError::InvalidMultibase);
    }

    #[test]
    fn content_id_from_content() {
        let content = b"Hello, Mosaic Trust Network!";
        let cid1 = ContentId::from_content(content);
        let cid2 = ContentId::from_content(content);
        assert_eq!(cid1, cid2);

        let cid3 = ContentId::from_content(b"Different content");
        assert_ne!(cid1, cid3);
    }

    #[test]
    fn key_id_from_str() {
        let kid = KeyId::from_str_id("key-1").unwrap();
        assert_eq!(kid.as_str(), Some("key-1"));
    }

    #[test]
    fn key_id_max_length() {
        let long = vec![b'a'; 65];
        assert!(KeyId::new(long).is_err());

        let ok = vec![b'a'; 64];
        assert!(KeyId::new(ok).is_ok());
    }
}
