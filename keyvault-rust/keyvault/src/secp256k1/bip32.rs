use super::{Bip178, Secp256k1, SecpExtPrivateKey, SecpExtPublicKey};
use crate::{bip32, Bip32Node, Bip32PublicNode, Network};

use super::*;

impl Bip32Node<Secp256k1> {
    /// Recreates the BIP32 node from its parts
    pub fn from_xprv(
        path: bip32::Path, xprv: impl AsRef<str>, network: &'static dyn Network<Suite = Secp256k1>,
    ) -> Result<Bip32Node<Secp256k1>> {
        let xsk = SecpExtPrivateKey::from_xprv(xprv.as_ref(), network.bip32_xprv())?;
        Ok(Bip32Node::new(path, xsk, network.subtree()))
    }

    /// Returns the extended private key in the BIP32 readable format with the version bytes of the network.
    pub fn to_xprv(&self, network: &'static dyn Network<Suite = Secp256k1>) -> String {
        let version = network.bip32_xprv();
        self.xsk().to_xprv(version)
    }

    /// Returns the private key in the Wallet Import Format with the version byte of the network.
    pub fn to_wif(&self, network: &'static dyn Network<Suite = Secp256k1>) -> String {
        let version = network.wif();
        self.private_key().to_wif(version, Bip178::Compressed)
    }
}

impl Bip32PublicNode<Secp256k1> {
    /// Recreates the BIP32 public node from its parts
    pub fn from_xpub(
        path: bip32::Path, xpub: impl AsRef<str>, network: &'static dyn Network<Suite = Secp256k1>,
    ) -> Result<Bip32PublicNode<Secp256k1>> {
        let xpk = SecpExtPublicKey::from_xpub(xpub.as_ref(), network.bip32_xpub())?;
        Ok(Bip32PublicNode::new(path, xpk, network.subtree()))
    }

    /// Returns the extended public key in the BIP32 readable format with the version bytes of the network.
    pub fn to_xpub(&self, network: &'static dyn Network<Suite = Secp256k1>) -> String {
        let version = network.bip32_xpub();
        self.xpk().to_xpub(version)
    }

    /// Returns the P2PKH address that belongs key with the version byte of the network.
    pub fn to_p2pkh_addr(&self, network: &'static dyn Network<Suite = Secp256k1>) -> String {
        let prefix = network.p2pkh_addr();
        self.key_id().to_p2pkh_addr(prefix)
    }
}
