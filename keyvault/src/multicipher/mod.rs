//! A type-erased version of [`AsymmetricCrypto`] and [`KeyDerivationCrypto`]. Serialized versions
//! of crypto concepts, like [`KeyId`], [`PublicKey`], [`PrivateKey`], [`Signature`],
//! [`ExtendedPrivateKey`] and [`ExtendedPublicKey`] can be all deserialized into
//! their [`MultiCipher`] versions.
//! This allows multiple cryptographic algorithms to co-exist in a software, which is needed
//! during migration of a single software to a new cryptography, or which is the status quo in
//! larger software ecosystems.
//!
//! [`MultiCipher`] can be thought of a variant of multiple incompatible cipher suits, which are
//! strongly typed, but are chosen at run-time.
//!
//! [`MultiCipher`]: struct.MultiCipher.html
//! [`AsymmetricCrypto`]: ../trait.AsymmetricCrypto.html
//! [`KeyDerivationCrypto`]: ../trait.KeyDerivationCrypto.html
//! [`KeyId`]: ../trait.AsymmetricCrypto.html#associatedtype.KeyId
//! [`PublicKey`]: ../trait.PublicKey.html
//! [`PrivateKey`]: ../trait.PrivateKey.html
//! [`Signature`]: ../trait.AsymmetricCrypto.html#associatedtype.Signature
//! [`ExtendedPrivateKey`]: ../trait.ExtendedPrivateKey.html
//! [`ExtendedPublicKey`]: ../trait.ExtendedPublicKey.html

mod id;
mod pk;
mod sig;
mod sk;

use super::*;

use std::hash::Hash;

use crate::ed25519::{EdKeyId, EdPrivateKey, EdPublicKey, EdSignature};
use crate::secp256k1::{SecpKeyId, SecpPrivateKey, SecpPublicKey, SecpSignature};
use crate::{AsymmetricCrypto, PrivateKey, PublicKey};

pub use id::MKeyId;
pub use pk::MPublicKey;
pub use sig::MSignature;
pub use sk::MPrivateKey;

/// A suite type that is used to keep the type-safety of the erased types in [`multicipher`]
///
/// [`multicipher`]: index.html
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum CipherSuite {
    /// The object tagged with this variant belongs to the [`ed25519`] module
    ///
    /// [`ed25519`]: ../ed25519/index.html
    Ed25519,
    /// The object tagged with this variant belongs to the [`secp256k1`] module
    ///
    /// [`secp256k1`]: ../secp256k1/index.html
    Secp256k1,
}

impl CipherSuite {
    fn as_byte(&self) -> u8 {
        match self {
            Self::Ed25519 => b'e',
            Self::Secp256k1 => b's',
        }
    }

    fn as_char(&self) -> char {
        self.as_byte() as char
    }

    fn from_char(c: char) -> Result<Self> {
        match c {
            'e' => Ok(Self::Ed25519),
            's' => Ok(Self::Secp256k1),
            _ => bail!("Unknown crypto suite '{}'", c),
        }
    }
}

#[derive(Clone, Debug)]
/// See the [module-level description](index.html).
pub struct MultiCipher;

impl AsymmetricCrypto for MultiCipher {
    type KeyId = MKeyId;
    type PublicKey = MPublicKey;
    type PrivateKey = MPrivateKey;
    type Signature = MSignature;
}

#[derive(Serialize, Deserialize)]
struct ErasedBytes {
    #[serde(rename = "s")]
    suite: u8,
    #[serde(rename = "v", with = "serde_bytes")]
    value: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let _cipher = MultiCipher {};
    }
}
