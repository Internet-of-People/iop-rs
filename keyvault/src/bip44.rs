use failure::{ensure, Fallible};

use super::*;

/// Entry point to generate a hierarchical deterministic wallet using the BIP-0044 standard. It is a more structured
/// way to use the same seed for multiple coins, each with multiple accounts, each accounts with a new key for each
/// transaction request. The standard is built on BIP-0043 using the purpose code 44. And BIP-0043 itself uses
/// BIP-0032 to derive all nodes from a single master extended private key.
#[derive(Clone, Debug)]
pub struct Bip44;

impl Bip44 {
    /// Creates the entry point to a given coin based on the subtree defined by SLIP-0044 for it.
    pub fn network<C: KeyDerivationCrypto>(
        &self, seed: &Seed, network: &'static dyn Network<Suite = C>,
    ) -> Fallible<Bip44Coin<C>> {
        let path = Bip44Path::coin(network.slip44());
        let master = Bip32.master(seed, network.subtree());
        let node = master.derive_hardened(44)?.derive_hardened(network.slip44())?;
        Ok(Bip44Coin { path, network, node })
    }
}

#[derive(Clone, Debug)]
/// Represents a given coin in the BIP32 tree.
pub struct Bip44Coin<C: KeyDerivationCrypto + 'static> {
    path: Bip44CoinPath,
    network: &'static dyn Network<Suite = C>,
    node: Bip32Node<C>,
}

impl<C: KeyDerivationCrypto + 'static> Bip44Coin<C> {
    /// Backdoor to mount a full BIP44 hierarchy at any place in the BIP32 tree. Use only for
    /// implementing advanced or fringe non-standard use-cases.
    pub fn new(
        path: Bip44CoinPath, network: &'static dyn Network<Suite = C>, node: Bip32Node<C>,
    ) -> Self {
        Self { path, network, node }
    }

    /// Accessor for the underlying BIP32 node.
    pub fn node(&self) -> &Bip32Node<C> {
        &self.node
    }

    /// Creates an account in the coin with the given index.
    pub fn account(&self, account: i32) -> Fallible<Bip44Account<C>> {
        ensure!(account >= 0, "Cannot use negative account index");
        Ok(Bip44Account::new(
            self.path.clone().account(account),
            self.network,
            self.node.derive_hardened(account)?,
        ))
    }

    /// Accessor for the BIP44 path of the coin.
    pub fn bip44_path(&self) -> &Bip44CoinPath {
        &self.path
    }

    /// Accessor for the BIP32 path of the coin.
    pub fn bip32_path(&self) -> &bip32::Path {
        self.node.path()
    }

    /// Accessor for the underlying nwtwork.
    pub fn network(&self) -> &'static dyn Network<Suite = C> {
        self.network
    }
}

#[derive(Clone, Debug)]
/// Represents a given account of a given coin in the BIP32 tree.
pub struct Bip44Account<C: KeyDerivationCrypto + 'static> {
    path: Bip44AccountPath,
    network: &'static dyn Network<Suite = C>,
    node: Bip32Node<C>,
}

impl<C: KeyDerivationCrypto + 'static> Bip44Account<C> {
    /// Backdoor to mount a BIP44 account at any place in the BIP32 tree. Use only for
    /// implementing advanced or fringe non-standard use-cases.
    pub fn new(
        path: Bip44AccountPath, network: &'static dyn Network<Suite = C>, node: Bip32Node<C>,
    ) -> Self {
        Self { path, network, node }
    }

    /// Accessor for the underlying BIP32 node.
    pub fn node(&self) -> &Bip32Node<C> {
        &self.node
    }

    /// Creates a sub-account for either external keys (receiving addresses) or
    /// internal keys (change addresses). This distinction might help in accounting.
    pub fn chain(&self, chain: Chain) -> Fallible<Bip44SubAccount<C>> {
        Ok(Bip44SubAccount::new(
            self.path.clone().chain(chain),
            self.network,
            self.node.derive_normal(chain as i32)?,
        ))
    }

    /// Creates a key with a given index used on the chain for storing balance or
    /// authenticating actions. By default these keys are made on the receiving sub-account.
    pub fn key(&self, key: i32) -> Fallible<Bip44Key<C>> {
        self.chain(Chain::Receiving)?.key(key)
    }

    /// Accessor for the BIP44 path of the account.
    pub fn bip44_path(&self) -> &Bip44AccountPath {
        &self.path
    }

    /// Accessor for the BIP32 path of the account.
    pub fn bip32_path(&self) -> &bip32::Path {
        self.node.path()
    }

    /// Neuters the account and converts it into its public API
    pub fn neuter(&self) -> Bip44PublicAccount<C> {
        Bip44PublicAccount::new(self.path.clone(), self.network, self.node.neuter())
    }

    /// Accessor for the underlying nwtwork.
    pub fn network(&self) -> &'static dyn Network<Suite = C> {
        self.network
    }
}

#[derive(Clone, Debug)]
/// Represents a given account of a given coin in the BIP32 tree.
pub struct Bip44PublicAccount<C: KeyDerivationCrypto + 'static> {
    path: Bip44AccountPath,
    network: &'static dyn Network<Suite = C>,
    node: Bip32PublicNode<C>,
}

impl<C: KeyDerivationCrypto + 'static> Bip44PublicAccount<C> {
    /// Backdoor to mount a BIP44 account at any place in the BIP32 tree. Use only for
    /// implementing advanced or fringe non-standard use-cases.
    pub fn new(
        path: Bip44AccountPath, network: &'static dyn Network<Suite = C>, node: Bip32PublicNode<C>,
    ) -> Self {
        Self { path, network, node }
    }

    /// Accessor for the underlying BIP32 node.
    pub fn node(&self) -> &Bip32PublicNode<C> {
        &self.node
    }

    /// Creates a sub-account for either external keys (receiving addresses) or
    /// internal keys (change addresses). This distinction might help in accounting.
    pub fn chain(&self, chain: Chain) -> Fallible<Bip44PublicSubAccount<C>> {
        Ok(Bip44PublicSubAccount::new(
            self.path.clone().chain(chain),
            self.network,
            self.node.derive_normal(chain as i32)?,
        ))
    }

    /// Creates a key with a given index used on the chain for storing balance or
    /// authenticating actions. By default these keys are made on the receiving sub-account.
    pub fn key(&self, key: i32) -> Fallible<Bip44PublicKey<C>> {
        self.chain(Chain::Receiving)?.key(key)
    }

    /// Accessor for the BIP44 path of the account.
    pub fn bip44_path(&self) -> &Bip44AccountPath {
        &self.path
    }

    /// Accessor for the BIP32 path of the account.
    pub fn bip32_path(&self) -> &bip32::Path {
        self.node.path()
    }

    /// Accessor for the underlying nwtwork.
    pub fn network(&self) -> &'static dyn Network<Suite = C> {
        self.network
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
/// Enumeration used for distinguishing external keys (receiving addresses) from
/// internal keys (change addresses). This distinction might help in accounting.
pub enum Chain {
    /// External keys (receiving addresses)
    Receiving = 0,
    /// Internal keys (change addresses)
    Change = 1,
}

impl From<bool> for Chain {
    fn from(change: bool) -> Self {
        if change {
            Chain::Change
        } else {
            Chain::Receiving
        }
    }
}

impl Into<bool> for Chain {
    fn into(self) -> bool {
        matches!(self, Chain::Change)
    }
}

#[derive(Clone, Debug)]
/// A sub-account of a given account on a given coin that is either used for external keys (receiving addresses) or
/// internal keys (change addresses). Some implementations do not distinguish these and just always use receiving
/// addresses.
pub struct Bip44SubAccount<C: KeyDerivationCrypto + 'static> {
    path: Bip44SubAccountPath,
    network: &'static dyn Network<Suite = C>,
    node: Bip32Node<C>,
}

impl<C: KeyDerivationCrypto + 'static> Bip44SubAccount<C> {
    /// Backdoor to mount a BIP44 sub-account at any place in the BIP32 tree. Use only for
    /// implementing advanced or fringe non-standard use-cases.
    pub fn new(
        path: Bip44SubAccountPath, network: &'static dyn Network<Suite = C>, node: Bip32Node<C>,
    ) -> Self {
        Self { path, network, node }
    }

    /// Accessor for the underlying BIP32 node.
    pub fn node(&self) -> &Bip32Node<C> {
        &self.node
    }

    /// Creates a key with a given index used on the chain for storing balance or
    /// authenticating actions.
    pub fn key(&self, key: i32) -> Fallible<Bip44Key<C>> {
        Ok(Bip44Key::new(self.path.clone().key(key), self.network, self.node.derive_normal(key)?))
    }

    /// Accessor for the BIP44 path of the sub-account.
    pub fn bip44_path(&self) -> &Bip44SubAccountPath {
        &self.path
    }

    /// Accessor for the BIP32 path of the sub-account.
    pub fn bip32_path(&self) -> &bip32::Path {
        self.node.path()
    }

    /// Neuters the sub-account and converts it into its public API
    pub fn neuter(&self) -> Bip44PublicSubAccount<C> {
        Bip44PublicSubAccount::new(self.path.clone(), self.network, self.node.neuter())
    }

    /// Accessor for the underlying nwtwork.
    pub fn network(&self) -> &'static dyn Network<Suite = C> {
        self.network
    }
}

#[derive(Clone, Debug)]
/// A sub-account of a given account on a given coin that is either used for external keys (receiving addresses) or
/// internal keys (change addresses). Some implementations do not distinguish these and just always use receiving
/// addresses.
pub struct Bip44PublicSubAccount<C: KeyDerivationCrypto + 'static> {
    path: Bip44SubAccountPath,
    network: &'static dyn Network<Suite = C>,
    node: Bip32PublicNode<C>,
}

impl<C: KeyDerivationCrypto + 'static> Bip44PublicSubAccount<C> {
    /// Backdoor to mount a BIP44 sub-account at any place in the BIP32 tree. Use only for
    /// implementing advanced or fringe non-standard use-cases.
    pub fn new(
        path: Bip44SubAccountPath, network: &'static dyn Network<Suite = C>,
        node: Bip32PublicNode<C>,
    ) -> Self {
        Self { path, network, node }
    }

    /// Accessor for the underlying BIP32 node.
    pub fn node(&self) -> &Bip32PublicNode<C> {
        &self.node
    }

    /// Creates a key with a given index used on the chain for storing balance or
    /// authenticating actions.
    pub fn key(&self, key: i32) -> Fallible<Bip44PublicKey<C>> {
        Ok(Bip44PublicKey::new(
            self.path.clone().key(key),
            self.network,
            self.node.derive_normal(key)?,
        ))
    }

    /// Accessor for the BIP44 path of the sub-account.
    pub fn bip44_path(&self) -> &Bip44SubAccountPath {
        &self.path
    }

    /// Accessor for the BIP32 path of the sub-account.
    pub fn bip32_path(&self) -> &bip32::Path {
        self.node.path()
    }

    /// Accessor for the underlying nwtwork.
    pub fn network(&self) -> &'static dyn Network<Suite = C> {
        self.network
    }
}

#[derive(Clone, Debug)]
/// Represents a key with a given index used on the chain for storing balance or authenticating actions.
pub struct Bip44Key<C: KeyDerivationCrypto + 'static> {
    path: Bip44KeyPath,
    network: &'static dyn Network<Suite = C>,
    node: Bip32Node<C>,
}

impl<C: KeyDerivationCrypto + 'static> Bip44Key<C> {
    /// Backdoor to mount a BIP44 key at any place in the BIP32 tree. Use only for
    /// implementing advanced or fringe non-standard use-cases.
    pub fn new(
        path: Bip44KeyPath, network: &'static dyn Network<Suite = C>, node: Bip32Node<C>,
    ) -> Self {
        Self { path, network, node }
    }

    /// Accessor for the underlying BIP32 node.
    pub fn node(&self) -> &Bip32Node<C> {
        &self.node
    }

    /// Accessor for the BIP44 path of the key.
    pub fn bip44_path(&self) -> &Bip44KeyPath {
        &self.path
    }

    /// Accessor for the BIP32 path of the key.
    pub fn bip32_path(&self) -> &bip32::Path {
        self.node.path()
    }

    /// Creates the private key for authenticating actions.
    pub fn to_private_key(&self) -> C::PrivateKey {
        self.node.private_key()
    }

    /// Neuters the key and convert it into its public API
    pub fn neuter(&self) -> Bip44PublicKey<C> {
        Bip44PublicKey::new(self.path.clone(), self.network, self.node.neuter())
    }

    /// Accessor for the underlying nwtwork.
    pub fn network(&self) -> &'static dyn Network<Suite = C> {
        self.network
    }
}

#[derive(Clone, Debug)]
/// Represents a public key with a given index used on the chain for verifying signatures or validating key identifiers.
pub struct Bip44PublicKey<C: KeyDerivationCrypto + 'static> {
    path: Bip44KeyPath,
    network: &'static dyn Network<Suite = C>,
    node: Bip32PublicNode<C>,
}

impl<C: KeyDerivationCrypto + 'static> Bip44PublicKey<C> {
    /// Backdoor to mount a BIP44 key at any place in the BIP32 tree. Use only for
    /// implementing advanced or fringe non-standard use-cases.
    pub fn new(
        path: Bip44KeyPath, network: &'static dyn Network<Suite = C>, node: Bip32PublicNode<C>,
    ) -> Self {
        Self { path, network, node }
    }

    /// Accessor for the underlying BIP32 node.
    pub fn node(&self) -> &Bip32PublicNode<C> {
        &self.node
    }

    /// Accessor for the BIP44 path of the key.
    pub fn bip44_path(&self) -> &Bip44KeyPath {
        &self.path
    }

    /// Accessor for the BIP32 path of the key.
    pub fn bip32_path(&self) -> &bip32::Path {
        self.node.path()
    }

    /// Creates the public key for verifying authentications done by this key.
    pub fn to_public_key(&self) -> C::PublicKey {
        self.node.public_key()
    }

    /// Creates the key identifier for the public key. This is an extra layer of security for single-use keys, so the
    /// revealing of the public key can be delayed to the point when the authenticated action (spending some coin or
    /// revoking access) makes the public key irrelevant after the action is successful.
    pub fn to_key_id(&self) -> C::KeyId {
        self.node.key_id()
    }

    /// Accessor for the underlying nwtwork.
    pub fn network(&self) -> &'static dyn Network<Suite = C> {
        self.network
    }
}

// pub enum AccountType {
//     Admin = 0,
//     Device = 1,
//     Persona = 2,
//     Group = 3,
// }

#[cfg(test)]
mod test {
    use super::{
        multicipher::*,
        secp256k1::{ark, hyd},
        *,
    };

    const PHRASE: &str = "blast cargo razor option vote shoe stock cruel mansion boy spot never album crop reflect kangaroo blouse slam empty shoot cable vital crane manual";

    #[test]
    fn path() -> Fallible<()> {
        let seed = Bip39::new().phrase(PHRASE)?.password("");

        let key = Bip44.network(&seed, &ark::Mainnet)?.account(0)?.chain(Chain::Change)?.key(0)?;
        let expected = "m/44'/111'/0'/1/0".parse::<Path>()?;
        assert_eq!(key.bip32_path(), &expected);
        Ok(())
    }

    #[test]
    fn derive() -> Fallible<()> {
        let seed = Bip39::new().phrase(PHRASE)?.password(Seed::PASSWORD);
        let key =
            Bip44.network(&seed, &hyd::Mainnet)?.account(0)?.chain(Chain::Receiving)?.key(0)?;

        assert_eq!(key.neuter().to_p2pkh_addr(), "hWNN8ymcsLdJivbwbBaPS8X1vekxB2pdwV");
        Ok(())
    }

    #[test]
    fn bip44_fantasy() -> Fallible<()> {
        let seed = Bip39::new().phrase(PHRASE)?.password(Seed::PASSWORD);

        let coin = Bip44.network(&seed, &hyd::Mainnet)?;
        assert_eq!(coin.bip32_path(), &"m/44'/4741444'".parse()?);
        assert_eq!(coin.to_xprv(), "HYDMVzSE83q9tomV2q935JF5mRZXUB37urS6ZpAvi17d5tFNK7K45ddZmjMZJKYQ8yLjKrq4HFewhXuL5AjDzb9Ft5efNT8upEy1ftARyhmxRFZH");

        let account = coin.account(0)?;
        assert_eq!(account.bip32_path(), &"m/44'/4741444'/0'".parse()?);
        assert_eq!(account.to_xprv(), "HYDMVzUVgP7S8GNPrKWhvoFPivfS25QnhfKi1iydA7jbWRwVuMJTVZzBQvBV86zpNJg83rrtvj6SWsftT3nNg5PQ9kwEdzTSpEDH5KbZsjbKBbhs");
        assert_eq!(account.neuter().to_xpub(), "hydmW129yVihVKVgXGWfvV3ZSePrhri2FgepKuyMEZ3Eg7bf91jrFZrmAo2hsVcS49dTRP5wZM7A8vudxWk5n8J2Ci12CSNvLy76CsnRaMK4A2b5");

        let key = account.key(5)?;
        assert_eq!(key.bip32_path(), &"m/44'/4741444'/0'/0/5".parse()?);
        assert_eq!(key.to_wif(), "HU6abSVnUK71QybJeoMNzriuLj7snTyzwM1vU1EtQVPxz9zVvBha");
        assert_eq!(
            hex::encode(key.to_private_key().to_bytes()),
            "51957a95a577cba15e1c4d4c4683bcaab17e03fbe7df12c60e69bf263485a00c"
        );
        assert_eq!(
            MPrivateKey::from(key.to_private_key()).public_key().to_string(),
            "pszxZN1JfKTPiZ2EfEzGW6eFmMnPV9gnUGCSTwHKyFbHAVK"
        );
        assert_eq!(
            hex::encode(key.neuter().to_public_key().to_bytes()),
            "03397d1e17dbecc754b9b1089ae2cce315def5e24df9a9b82230b730418e80110e"
        );
        assert_eq!(
            MPublicKey::from(key.neuter().to_public_key()).to_string(),
            "pszxZN1JfKTPiZ2EfEzGW6eFmMnPV9gnUGCSTwHKyFbHAVK"
        );
        assert_eq!(key.neuter().to_p2pkh_addr(), "hKQGzsR5sStKMEcX1EtGKa3Q33jWH6JfKC");
        assert_eq!(
            MKeyId::from(key.neuter().to_key_id()).to_string(),
            "isz5GewAWjJGfSB31qzds34UUSpnsmD"
        );

        let sub_account = account.chain(Chain::Receiving)?;
        assert_eq!(sub_account.bip32_path(), &"m/44'/4741444'/0'/0".parse()?);
        assert_eq!(sub_account.to_xprv(), "HYDMVzVXwQGdZsas8KUKcmTzXtR4Le1fioCn4r8aMFa1CLvCzXJJa28ogEUPdZD1BrSTuBLDGRv5SaPHStH7ZYABhdD6Tf6dCPLd5eaJ9aArC8po");
        assert_eq!(sub_account.neuter().to_xpub(), "hydmW13CEWstvvi9oGUHcTGAFc9V2RJuGpXtP38JRgseN2aNEBjhL21PS7KcNwqbz2jh787bMSPNkd7sYsdVm5rzdeyweau32iL6qzCgpn79R4o2");

        let recv0 = sub_account.key(0)?;
        assert_eq!(recv0.bip32_path(), &"m/44'/4741444'/0'/0/0".parse()?);
        assert_eq!(recv0.neuter().to_p2pkh_addr(), "hWNN8ymcsLdJivbwbBaPS8X1vekxB2pdwV");

        let change2 = account.chain(Chain::Change)?.key(2)?;
        assert_eq!(change2.bip32_path(), &"m/44'/4741444'/0'/1/2".parse()?);

        // let pub_account = account.neuter();

        Ok(())
    }
}
