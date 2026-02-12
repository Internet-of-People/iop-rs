//! Multi-algorithm signature types for `did:mosaic`.
//!
//! Supports Ed25519 (required) and secp256k1 (Ethereum compatibility).
//! Algorithm agility enables future post-quantum additions (Dilithium).

use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

/// Multi-algorithm signature supporting Ed25519 and secp256k1.
///
/// Designed for algorithm agility — new variants can be added for
/// post-quantum algorithms (e.g., Dilithium) without breaking changes.
///
/// Signatures are stored as `Vec<u8>` with runtime length validation
/// to support serde across all array sizes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "substrate", derive(parity_scale_codec::Encode, parity_scale_codec::Decode, scale_info::TypeInfo))]
pub enum MultiSignature {
    /// Ed25519 signature (64 bytes).
    Ed25519(Vec<u8>),
    /// secp256k1 signature (65 bytes, recoverable).
    Secp256k1(Vec<u8>),
}

impl MultiSignature {
    /// Create an Ed25519 signature from a 64-byte array.
    pub fn ed25519(sig: [u8; 64]) -> Self {
        MultiSignature::Ed25519(sig.to_vec())
    }

    /// Create a secp256k1 signature from a 65-byte array.
    pub fn secp256k1(sig: [u8; 65]) -> Self {
        MultiSignature::Secp256k1(sig.to_vec())
    }

    /// Returns the raw signature bytes.
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            MultiSignature::Ed25519(sig) => sig,
            MultiSignature::Secp256k1(sig) => sig,
        }
    }

    /// Returns the algorithm name.
    pub fn algorithm(&self) -> &'static str {
        match self {
            MultiSignature::Ed25519(_) => "Ed25519",
            MultiSignature::Secp256k1(_) => "secp256k1",
        }
    }

    /// Returns the expected signature length for this variant.
    pub fn expected_len(&self) -> usize {
        match self {
            MultiSignature::Ed25519(_) => 64,
            MultiSignature::Secp256k1(_) => 65,
        }
    }

    /// Validate that the signature bytes have the correct length.
    pub fn is_valid_length(&self) -> bool {
        self.as_bytes().len() == self.expected_len()
    }
}

/// Multi-algorithm public key for verification.
///
/// Public keys are stored as `Vec<u8>` with runtime length validation.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "substrate", derive(parity_scale_codec::Encode, parity_scale_codec::Decode, scale_info::TypeInfo))]
pub enum MultiPublicKey {
    /// Ed25519 public key (32 bytes).
    Ed25519(Vec<u8>),
    /// secp256k1 compressed public key (33 bytes).
    Secp256k1(Vec<u8>),
}

impl MultiPublicKey {
    /// Create an Ed25519 public key from a 32-byte array.
    pub fn ed25519(pk: [u8; 32]) -> Self {
        MultiPublicKey::Ed25519(pk.to_vec())
    }

    /// Create a secp256k1 public key from a 33-byte array.
    pub fn secp256k1(pk: [u8; 33]) -> Self {
        MultiPublicKey::Secp256k1(pk.to_vec())
    }

    /// Returns the raw public key bytes.
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            MultiPublicKey::Ed25519(pk) => pk,
            MultiPublicKey::Secp256k1(pk) => pk,
        }
    }

    /// Returns the algorithm name.
    pub fn algorithm(&self) -> &'static str {
        match self {
            MultiPublicKey::Ed25519(_) => "Ed25519",
            MultiPublicKey::Secp256k1(_) => "secp256k1",
        }
    }

    /// Returns the expected key length for this variant.
    pub fn expected_len(&self) -> usize {
        match self {
            MultiPublicKey::Ed25519(_) => 32,
            MultiPublicKey::Secp256k1(_) => 33,
        }
    }

    /// Validate that the key bytes have the correct length.
    pub fn is_valid_length(&self) -> bool {
        self.as_bytes().len() == self.expected_len()
    }

    /// Verify a signature against this public key and message.
    ///
    /// Returns `true` if the signature is valid for the given message.
    pub fn verify(&self, message: &[u8], signature: &MultiSignature) -> bool {
        match (self, signature) {
            (MultiPublicKey::Ed25519(pk), MultiSignature::Ed25519(sig)) => {
                if pk.len() != 32 || sig.len() != 64 {
                    return false;
                }
                let mut pk_arr = [0u8; 32];
                pk_arr.copy_from_slice(pk);
                let mut sig_arr = [0u8; 64];
                sig_arr.copy_from_slice(sig);
                verify_ed25519(&pk_arr, message, &sig_arr)
            }
            (MultiPublicKey::Secp256k1(_pk), MultiSignature::Secp256k1(_sig)) => {
                // secp256k1 verification requires external crate — stubbed for now
                // Will be implemented when integrating with iop-keyvault's secp256k1 support
                false
            }
            _ => false, // Algorithm mismatch
        }
    }
}

/// Ed25519 signature verification using iop-keyvault's implementation.
fn verify_ed25519(public_key: &[u8; 32], message: &[u8], signature: &[u8; 64]) -> bool {
    use iop_keyvault::ed25519;

    let pk = match ed25519::EdPublicKey::from_bytes(public_key) {
        Ok(pk) => pk,
        Err(_) => return false,
    };
    let sig = match ed25519::EdSignature::from_bytes(signature) {
        Ok(sig) => sig,
        Err(_) => return false,
    };

    use iop_keyvault::PublicKey;
    pk.verify(message, &sig)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_signature_ed25519() {
        let sig = MultiSignature::ed25519([0u8; 64]);
        assert_eq!(sig.algorithm(), "Ed25519");
        assert_eq!(sig.as_bytes().len(), 64);
        assert!(sig.is_valid_length());
    }

    #[test]
    fn multi_signature_secp256k1() {
        let sig = MultiSignature::secp256k1([0u8; 65]);
        assert_eq!(sig.algorithm(), "secp256k1");
        assert_eq!(sig.as_bytes().len(), 65);
        assert!(sig.is_valid_length());
    }

    #[test]
    fn multi_public_key_algorithm() {
        let pk_ed = MultiPublicKey::ed25519([0u8; 32]);
        assert_eq!(pk_ed.algorithm(), "Ed25519");
        assert!(pk_ed.is_valid_length());

        let pk_secp = MultiPublicKey::secp256k1([0u8; 33]);
        assert_eq!(pk_secp.algorithm(), "secp256k1");
        assert!(pk_secp.is_valid_length());
    }

    #[test]
    fn algorithm_mismatch_returns_false() {
        let pk = MultiPublicKey::ed25519([0u8; 32]);
        let sig = MultiSignature::secp256k1([0u8; 65]);
        assert!(!pk.verify(b"test", &sig));
    }

    #[test]
    fn invalid_length_signature() {
        let sig = MultiSignature::Ed25519(vec![0u8; 32]); // Too short
        assert!(!sig.is_valid_length());
    }
}
