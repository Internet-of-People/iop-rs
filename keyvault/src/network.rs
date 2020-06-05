use std::fmt;

use super::*;

/// It might sound a bit pedantic, but some Network trait methods return fixed length byte array
/// static borrows instead of single bytes.
pub const ADDR_PREFIX_SIZE: usize = 1;

/// Extended public and private keys use version bytes to help finding out how these keys are
/// used on the blockchain and which blockchains they are used on
pub const BIP32_VERSION_PREFIX_SIZE: usize = 4;

/// The operations required to support SLIP-0010
pub trait Subtree {
    /// The crypto suite used by the network
    type Suite: KeyDerivationCrypto;

    /// the name of the network shown on the UI or in Debug info
    fn name(&self) -> &'static str;

    /// Calculate the master extended private key in the crypto suite used by the network
    fn master(
        &self, seed: &Seed,
    ) -> <<Self as Subtree>::Suite as KeyDerivationCrypto>::ExtendedPrivateKey;

    /// Calculate the key identifier from the public key based on the rules used by the network
    fn key_id(
        &self, pk: &<<Self as Subtree>::Suite as AsymmetricCrypto>::PublicKey,
    ) -> <<Self as Subtree>::Suite as AsymmetricCrypto>::KeyId;
}

impl<C: KeyDerivationCrypto + 'static> fmt::Debug for &dyn Subtree<Suite = C> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        formatter.write_str(self.name())
    }
}

/// Strategy that can be implemented for different clones of the Bitcoin network. It is a trait
/// rather than an enumeration to leave it open for extensions outside this crate. A few example
/// implementations can be found under the network submodules.
pub trait Network: Subtree {
    /// `to_p2pkh_addr` needs a prefix
    fn p2pkh_addr(&self) -> &'static [u8; ADDR_PREFIX_SIZE];
    /// In the future p2sh will need a prefix
    fn p2sh_addr(&self) -> &'static [u8; ADDR_PREFIX_SIZE];
    /// `to_wif` and `from_wif` needs a prefix
    fn wif(&self) -> &'static [u8; ADDR_PREFIX_SIZE];
    /// `to_xprv` and `from_xprv` needs version bytes
    fn bip32_xprv(&self) -> &'static [u8; BIP32_VERSION_PREFIX_SIZE];
    /// `to_xpub` and `from_xpub` needs version bytes
    fn bip32_xpub(&self) -> &'static [u8; BIP32_VERSION_PREFIX_SIZE];
    /// signed free-text messages are prefixed with this text
    fn message_prefix(&self) -> &'static str;
    /// SLIP-44 registered coin number for BIP-44 derivation
    fn slip44(&self) -> i32;

    /// upcast the network to a subtree (each network implements subtree, too, but
    /// the compiler does not know their relations in the implementation in advance)
    fn subtree(&self) -> &dyn Subtree<Suite = <Self as Subtree>::Suite>;
}

impl<C: KeyDerivationCrypto + 'static> fmt::Debug for &dyn Network<Suite = C> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        formatter.write_str(self.name())
    }
}
