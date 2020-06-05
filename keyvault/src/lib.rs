#![warn(missing_docs)]

//! This library provides a high-level API to be used in Morpheus as a key-vault. It wraps multiple
//! cryptographic libraries to make it easier on the integrator.

mod bip32;
mod bip39;
mod bip43;
mod bip44;
mod bip44path;
mod cc;
pub mod ed25519;
pub mod multicipher;
mod network;
pub mod secp256k1;
mod seed;
#[cfg(test)]
mod test_crypto;
#[cfg(test)]
mod tests;

use failure::{bail, ensure, Fail, Fallible};

pub use ::bip39::ErrorKind as Bip39ErrorKind;
pub use ::bip39::Language as Bip39Language;
pub use hmac::Mac;

pub use crate::bip39::*;
pub use bip32::*;
pub use bip43::*;
pub use bip44::*;
pub use bip44path::*;
pub use network::*;
pub use seed::*;

/// A public key (also called shared key or pk in some literature) is that part of an asymmetric keypair
/// which can be used to verify the authenticity of the sender of a message or to encrypt a message that
/// can only be decrypted by a single recipient. In both cases this other party owns the [`PrivateKey`]
/// part of the keypair and never shares it with anyone else.
///
/// [`PrivateKey`]: trait.PrivateKey.html
pub trait PublicKey<C: AsymmetricCrypto + ?Sized>: Clone {
    /// Calculates the ID (also called fingerprint or address in some literature) of the public key. In
    /// some algorithms the public key is only revealed in point-to-point communications and a keypair is
    /// identified only by the digest of the public key in all other channels.
    fn key_id(&self) -> C::KeyId;

    /// We do not have multiple versions of KeyIds for the same multicipher public key, so for now this comparison is trivial. But when we
    /// introduce newer versions, we need to take the version of the `key_id` argument into account and calculate that possibly older version
    /// from `self`.
    // When implementing, consider timing attack for validation with key_id() generation
    fn validate_id(&self, id: &C::KeyId) -> bool;

    /// This method can be used to verify if a given signature for a message was made using the private
    /// key that belongs to this public key. See also [`PrivateKey::sign`]
    ///
    /// [`PrivateKey::sign`]: trait.PrivateKey.html#tymethod.sign
    fn verify<D: AsRef<[u8]>>(&self, data: D, sig: &C::Signature) -> bool;
}

/// A private key (also called secret key or sk in some literature) is the part of an asymmetric keypair
/// which is never shared with anyone. It is used to sign a message sent to any recipient or to decrypt a
/// message that was sent encrypted from any recipients.
pub trait PrivateKey<C: AsymmetricCrypto + ?Sized>: Clone {
    /// Calculates the [`PublicKey`] that belongs to this private key. These two keys together form an
    /// asymmetric keypair, where the private key cannot be calculated from the public key with a reasonable
    /// effort, but the public key can be calculated from the private key cheaply.
    ///
    /// [`PublicKey`]: trait.PublicKey.html
    fn public_key(&self) -> C::PublicKey;

    /// Calculates the signature of a message that can be then verified using [`PublicKey::verify`]
    ///
    /// [`PublicKey::verify`]: trait.PublicKey.html#tymethod.verify
    fn sign<D: AsRef<[u8]>>(&self, data: D) -> C::Signature;
}

/// An implementation of this trait defines a family of types that fit together perfectly to form a
/// cryptography using asymmetric keypairs.
pub trait AsymmetricCrypto {
    /// The ID (also called fingerprint or address in some literature) of the public key. See
    /// [`PublicKey::key_id`] for more details.
    ///
    /// [`PublicKey::key_id`]: trait.PublicKey.html#tymethod.key_id
    type KeyId: std::hash::Hash + Eq + Clone;

    /// See [`PublicKey`] for more details.
    ///
    /// [`PublicKey`]: trait.PublicKey.html
    type PublicKey: PublicKey<Self>;

    /// See [`PrivateKey`] for more details.
    ///
    /// [`PrivateKey`]: trait.PrivateKey.html
    type PrivateKey: PrivateKey<Self>;

    /// The signature of a given message with a given private key. Its size and representation is up
    /// to the implementation.
    type Signature: Clone;
}

/// The hashing algorithm used for deriving child keys in SLIP-0010
pub type HmacSha512 = hmac::Hmac<sha2::Sha512>;

/// Extended private key not only contains a private key, but also a chain code that is some additional entropy that
/// is used to derive child keys. Some cryptographic suites implement both normal (public) and hardened (private)
/// derivation, some, like Ed25519 is missing normal derivation and just err when called.
///
/// An extended private key can be neutered to an extended public key, which contains the same chain code, but its
/// public key part does not reveal any information about the private key.
pub trait ExtendedPrivateKey<C: KeyDerivationCrypto + ?Sized>: Clone {
    /// Normal derivation allows the neutered extended public key to calculate child extended public keys without
    /// revealing any private keys.
    fn derive_normal_child(&self, idx: i32) -> Fallible<C::ExtendedPrivateKey>;
    /// Hardened derivation makes it impossible to the neutered extended public key to calculate children. It uses
    /// a different derivation algorithm.
    fn derive_hardened_child(&self, idx: i32) -> Fallible<C::ExtendedPrivateKey>;
    /// Neutering an extended private key gives an extended public key that contains the private key neutered, plus
    /// the chain code. It is useless to reveal the chain code when hardened derivation is used.
    fn neuter(&self) -> C::ExtendedPublicKey;
    /// Throws away the chain code and gives back only the private key from the extended private key.
    fn private_key(&self) -> C::PrivateKey;
}

/// Extended public key is a neutered extended private key that contains the public key that belongs to the private key in that,
/// but it also contains the chain code so it can be used in normal (public) derivation. Some cryptographic suites do not have
/// normal derivation and those are free to implement extended public keys containing only the public key.
pub trait ExtendedPublicKey<C: KeyDerivationCrypto + ?Sized>: Clone {
    /// Derive child extended public keys. Useful for auditing hierarchical deterministic wallets, or generating a new address
    /// for each on-chain transaction knowing the owner of the corresponding extended private key can spend the received coins.
    fn derive_normal_child(&self, idx: i32) -> Fallible<C::ExtendedPublicKey>;
    /// Throws away the chain code and gives back only the public key from the extended public key.
    fn public_key(&self) -> C::PublicKey;
}

/// An implementation of this trait defines a family of types that fit together to extend [`AsymmetricCrypto`]
/// with the ability to have a hierarchical deterministic wallet, so a tree of private and public keys all
/// derived from a single master secret.
///
/// [`AsymmetricCrypto`]: trait.AsymmetricCrypto.html
pub trait KeyDerivationCrypto: AsymmetricCrypto {
    /// See [`ExtendedPrivateKey`] for more details.
    ///
    /// [`ExtendedPrivateKey`]: trait.ExtendedPrivateKey.html
    type ExtendedPrivateKey: ExtendedPrivateKey<Self>;
    /// See [`ExtendedPublicKey`] for more details.
    ///
    /// [`ExtendedPublicKey`]: trait.ExtendedPublicKey.html
    type ExtendedPublicKey: ExtendedPublicKey<Self>;

    /// Does not seem to completely belong here, but calculates the master extended private key - the root of a hierarchical
    /// deterministic wallet - from a given seed. All other keys are derived from this one extended private key.
    fn master(seed: &Seed) -> Self::ExtendedPrivateKey;
}

/// Unicode code point for planet mercury
pub const BIP43_PURPOSE_MERCURY: i32 = 0x263F;
