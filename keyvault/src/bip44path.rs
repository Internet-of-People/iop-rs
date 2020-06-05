use super::*;

/// Entry point to generate a [BIP-0044](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki) compliant key hierarchy.
pub struct Bip44Path;

impl Bip44Path {
    /// Creates a path for a BIP-0044 coin with a given coin index (check [SLIP-0044](https://github.com/satoshilabs/slips/blob/master/slip-0044.md)
    /// for registered coin indexes)
    pub fn coin(slip44: i32) -> Bip44CoinPath {
        Bip44CoinPath { slip44 }
    }
}

/// Valid operations for a bip44 coin.
#[derive(Clone, Debug)]
pub struct Bip44CoinPath {
    slip44: i32,
}

impl Bip44CoinPath {
    /// Accessor for the coin index.
    pub fn slip44(&self) -> i32 {
        self.slip44
    }

    /// Creates an account of the current coin.
    pub fn account(self, idx: i32) -> Bip44AccountPath {
        Bip44AccountPath { parent: self, account: idx }
    }

    /// Returns the BIP-0032 path of the coin.
    pub fn bip32_path(&self) -> bip32::Path {
        Bip43Path::purpose(44).bip32_path().append(ChildIndex::Hardened(self.slip44))
    }
}

/// Valid operations for a bip44 account.
#[derive(Clone, Debug)]
pub struct Bip44AccountPath {
    parent: Bip44CoinPath,
    account: i32,
}

impl Bip44AccountPath {
    /// Accessor for the coin this account is in.
    pub fn parent(&self) -> &Bip44CoinPath {
        &self.parent
    }

    /// Accessor for the account index.
    pub fn account(&self) -> i32 {
        self.account
    }

    /// Creates a sub-account of the current account. For blockchains with UTXO-based accounting
    /// it makes sense to separate change addresses from receiving addresses. For balance-based
    /// accounting, just use the `key` method.
    pub fn chain(self, chain: Chain) -> Bip44SubAccountPath {
        Bip44SubAccountPath { parent: self, chain }
    }

    /// Creates a receiving key of the current account. See also the `chain` method for UTXO-based
    /// blockchains.
    pub fn key(self, idx: i32) -> Bip44KeyPath {
        self.chain(Chain::Receiving).key(idx)
    }

    /// Returns the BIP-0032 path of the account.
    pub fn bip32_path(&self) -> bip32::Path {
        self.parent.bip32_path().append(ChildIndex::Hardened(self.account))
    }
}

#[derive(Clone, Debug)]
/// Valid operations for a bip44 sub-account.
pub struct Bip44SubAccountPath {
    parent: Bip44AccountPath,
    chain: Chain,
}

impl Bip44SubAccountPath {
    /// Accessor for the account this sub-account is in.
    pub fn parent(&self) -> &Bip44AccountPath {
        &self.parent
    }

    /// Accessor for the chain (receiving or change).
    pub fn chain(&self) -> Chain {
        self.chain
    }

    /// Creates a receiving key of the current sub-account.
    pub fn key(self, idx: i32) -> Bip44KeyPath {
        Bip44KeyPath { parent: self, key: idx }
    }

    /// Returns the BIP-0032 path of the sub-account.
    pub fn bip32_path(&self) -> bip32::Path {
        self.parent.bip32_path().append(ChildIndex::Normal(self.chain() as i32))
    }
}

#[derive(Clone, Debug)]
/// Valid operations for a bip44 key.
pub struct Bip44KeyPath {
    parent: Bip44SubAccountPath,
    key: i32,
}

impl Bip44KeyPath {
    /// Accessor for the sub-account this key is in.
    pub fn parent(&self) -> &Bip44SubAccountPath {
        &self.parent
    }

    /// Accessor for the key index.
    pub fn key(&self) -> i32 {
        self.key
    }

    /// Returns the BIP-0032 path of the key.
    pub fn bip32_path(&self) -> bip32::Path {
        self.parent.bip32_path().append(ChildIndex::Normal(self.key()))
    }
}
