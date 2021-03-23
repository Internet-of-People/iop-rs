use super::*;
use crate::bip44::*;

impl Bip44Coin<Secp256k1> {
    /// Returns the extended private key in the BIP32 readable format with the version bytes of the network.
    pub fn to_xprv(&self) -> String {
        self.node().to_xprv(self.network())
    }
}

impl Bip44Account<Secp256k1> {
    /// Recreates the private API of a BIP44 account from its parts
    pub fn from_xprv(
        account: i32, xprv: impl AsRef<str>, network: &'static dyn Network<Suite = Secp256k1>,
    ) -> Result<Bip44Account<Secp256k1>> {
        let path = Bip44Path::coin(network.slip44()).account(account);
        let node = Bip32Node::from_xprv(path.bip32_path(), xprv, network)?;
        Ok(Bip44Account::new(path, network, node))
    }

    /// Returns the extended private key in the BIP32 readable format with the version bytes of the network.
    pub fn to_xprv(&self) -> String {
        self.node().to_xprv(self.network())
    }
}

impl Bip44PublicAccount<Secp256k1> {
    /// Recreates the public API of a BIP44 account from its parts
    pub fn from_xpub(
        account: i32, xpub: impl AsRef<str>, network: &'static dyn Network<Suite = Secp256k1>,
    ) -> Result<Bip44PublicAccount<Secp256k1>> {
        let path = Bip44Path::coin(network.slip44()).account(account);
        let node = Bip32PublicNode::from_xpub(path.bip32_path(), xpub, network)?;
        Ok(Bip44PublicAccount::new(path, network, node))
    }

    /// Returns the extended public key in the BIP32 readable format with the version bytes of the network.
    pub fn to_xpub(&self) -> String {
        self.node().to_xpub(self.network())
    }
}

impl Bip44SubAccount<Secp256k1> {
    /// Recreates the private API of a BIP44 account from its parts
    pub fn from_xprv(
        account: i32, chain: Chain, xprv: impl AsRef<str>,
        network: &'static dyn Network<Suite = Secp256k1>,
    ) -> Result<Bip44SubAccount<Secp256k1>> {
        let path = Bip44Path::coin(network.slip44()).account(account).chain(chain);
        let node = Bip32Node::from_xprv(path.bip32_path(), xprv, network)?;
        Ok(Bip44SubAccount::new(path, network, node))
    }

    /// Returns the extended private key in the BIP32 readable format with the version bytes of the network.
    pub fn to_xprv(&self) -> String {
        self.node().to_xprv(self.network())
    }
}

impl Bip44PublicSubAccount<Secp256k1> {
    /// Recreates the public API of a BIP44 sub-account from its parts
    pub fn from_xpub(
        account: i32, chain: Chain, xpub: impl AsRef<str>,
        network: &'static dyn Network<Suite = Secp256k1>,
    ) -> Result<Bip44PublicSubAccount<Secp256k1>> {
        let path = Bip44Path::coin(network.slip44()).account(account).chain(chain);
        let node = Bip32PublicNode::from_xpub(path.bip32_path(), xpub, network)?;
        Ok(Bip44PublicSubAccount::new(path, network, node))
    }

    /// Returns the extended public key in the BIP32 readable format with the version bytes of the network.
    pub fn to_xpub(&self) -> String {
        self.node().to_xpub(self.network())
    }
}

impl Bip44Key<Secp256k1> {
    /// Returns the private key in the Wallet Import Format with the version byte of the network.
    pub fn to_wif(&self) -> String {
        self.node().to_wif(self.network())
    }
}

impl Bip44PublicKey<Secp256k1> {
    /// Returns the P2PKH address that belongs key with the version byte of the network.
    pub fn to_p2pkh_addr(&self) -> String {
        self.node().to_p2pkh_addr(self.network())
    }
}
