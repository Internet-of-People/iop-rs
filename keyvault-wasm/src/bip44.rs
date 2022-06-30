use super::*;

/// Entry point to generate a hierarchical deterministic wallet using the [BIP-0044
/// standard](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki). It is a more structured way to use the same seed for
/// multiple coins, each with multiple accounts, each accounts with a new key for each transaction request. The standard is built on
/// [BIP-0043](https://github.com/bitcoin/bips/blob/master/bip-0043.mediawiki) using the purpose code 44. And BIP-0043 itself uses
/// BIP-0032 to derive all nodes from a single master extended private key.
///
/// @see Bip32
#[wasm_bindgen(js_name = Bip44)]
#[derive(Clone, Debug)]
pub struct JsBip44;

#[wasm_bindgen(js_class = Bip44)]
impl JsBip44 {
    /// Creates the BIP32 root node for a given coin from the given seed based on the network.
    /// We use coin identifiers defined in [SLIP-0044](https://github.com/satoshilabs/slips/blob/master/slip-0044.md).
    ///
    /// @see validateNetworkName, Seed
    pub fn network(seed: &JsSeed, name: &str) -> Result<JsBip44Coin, JsValue> {
        let network = Networks::by_name(name).map_err_to_js()?;
        let coin = Bip44.network(seed.inner(), network).map_err_to_js()?;
        Ok(JsBip44Coin::from(coin))
    }
}

/// Represents a given coin in the BIP32 tree.
///
/// @see Bip32
#[wasm_bindgen(js_name = Bip44Coin)]
#[derive(Clone, Debug)]
pub struct JsBip44Coin {
    inner: Bip44Coin<Secp256k1>,
}

#[wasm_bindgen(js_class = Bip44Coin)]
impl JsBip44Coin {
    /// Returns the underlying {@link Bip32Node}.
    pub fn node(&self) -> JsBip32Node {
        JsBip32Node::from(self.inner.node().clone())
    }

    /// Accessor for the name of the underlying network.
    #[wasm_bindgen(getter)]
    pub fn network(&self) -> String {
        self.inner.network().subtree().name().to_owned()
    }

    /// Creates an account in the coin with the given account index.
    pub fn account(&self, account: i32) -> Result<JsBip44Account, JsValue> {
        let account = self.inner.account(account).map_err_to_js()?;
        Ok(JsBip44Account::from(account))
    }

    /// The coin identifier of the coin, defined in [SLIP-0044](https://github.com/satoshilabs/slips/blob/master/slip-0044.md).
    #[wasm_bindgen(getter = slip44)]
    pub fn slip44(&self) -> i32 {
        self.inner.bip44_path().slip44()
    }

    /// Accessor for the BIP32 path of the coin.
    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.bip32_path().to_string()
    }

    // Secp specific methods...

    /// Returns the extended private key in the BIP32 readable format with the version bytes of the network.
    ///
    /// This is a secret that must not be kept unencrypted in transit or in rest!
    #[wasm_bindgen(getter = xprv)]
    pub fn to_xprv(&self) -> String {
        self.inner.to_xprv()
    }
}

impl From<Bip44Coin<Secp256k1>> for JsBip44Coin {
    fn from(inner: Bip44Coin<Secp256k1>) -> Self {
        Self { inner }
    }
}

impl Wraps<Bip44Coin<Secp256k1>> for JsBip44Coin {
    fn inner(&self) -> &Bip44Coin<Secp256k1> {
        &self.inner
    }
}

/// Represents the private API of a given account of a given coin in the BIP32 tree.
#[wasm_bindgen(js_name = Bip44Account)]
#[derive(Clone, Debug)]
pub struct JsBip44Account {
    inner: Bip44Account<Secp256k1>,
}

#[wasm_bindgen(js_class = Bip44Account)]
impl JsBip44Account {
    /// Returns the underlying {@link Bip32Node}.
    pub fn node(&self) -> JsBip32Node {
        JsBip32Node::from(self.inner.node().clone())
    }

    /// Accessor for the name of the underlying network.
    #[wasm_bindgen(getter)]
    pub fn network(&self) -> String {
        self.inner.network().subtree().name().to_owned()
    }

    /// Creates a sub-account for either external keys (receiving addresses) or internal keys (change addresses). This distinction is
    /// a common practice that might help in accounting.
    pub fn chain(&self, change: bool) -> Result<JsBip44SubAccount, JsValue> {
        let account = self.inner.chain(Chain::from(change)).map_err_to_js()?;
        Ok(JsBip44SubAccount::from(account))
    }

    /// Creates a key with a given index used on the chain for storing balance or
    /// authenticating actions. By default these keys are made on the receiving sub-account.
    pub fn key(&self, idx: i32) -> Result<JsBip44Key, JsValue> {
        let sub_account = self.chain(false)?;
        sub_account.key(idx)
    }

    /// The coin identifier of the coin, defined in [SLIP-0044](https://github.com/satoshilabs/slips/blob/master/slip-0044.md).
    #[wasm_bindgen(getter = slip44)]
    pub fn slip44(&self) -> i32 {
        self.inner.bip44_path().parent().slip44()
    }

    /// Accessor for the account index.
    #[wasm_bindgen(getter = account)]
    pub fn account(&self) -> i32 {
        self.inner.bip44_path().account()
    }

    /// Accessor for the BIP32 path of the account.
    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.bip32_path().to_string()
    }

    /// Neuters the account and converts it into its public API
    pub fn neuter(&self) -> JsBip44PublicAccount {
        let inner = self.inner.neuter();
        JsBip44PublicAccount::from(inner)
    }

    // Secp specific methods...

    /// Recreates the private API of a BIP44 account from its parts
    #[wasm_bindgen(js_name = fromXprv)]
    pub fn from_xprv(account: i32, xprv: &str, network: &str) -> Result<JsBip44Account, JsValue> {
        let network = Networks::by_name(network).map_err_to_js()?;
        let inner = Bip44Account::from_xprv(account, xprv, network).map_err_to_js()?;
        Ok(JsBip44Account::from(inner))
    }

    /// Returns the extended private key in the BIP32 readable format with the version bytes of the network.
    ///
    /// This is a secret that must not be kept unencrypted in transit or in rest!
    #[wasm_bindgen(getter = xprv)]
    pub fn to_xprv(&self) -> String {
        self.inner.to_xprv()
    }
}

impl From<Bip44Account<Secp256k1>> for JsBip44Account {
    fn from(inner: Bip44Account<Secp256k1>) -> Self {
        Self { inner }
    }
}

impl Wraps<Bip44Account<Secp256k1>> for JsBip44Account {
    fn inner(&self) -> &Bip44Account<Secp256k1> {
        &self.inner
    }
}

/// Represents the public API of a given account of a given coin in the BIP32 tree.
#[wasm_bindgen(js_name = Bip44PublicAccount)]
#[derive(Clone, Debug)]
pub struct JsBip44PublicAccount {
    inner: Bip44PublicAccount<Secp256k1>,
}

#[wasm_bindgen(js_class = Bip44PublicAccount)]
impl JsBip44PublicAccount {
    /// Returns the underlying {@link Bip32PublicNode}.
    pub fn node(&self) -> JsBip32PublicNode {
        JsBip32PublicNode::from(self.inner.node().clone())
    }

    /// Accessor for the name of the underlying network.
    #[wasm_bindgen(getter)]
    pub fn network(&self) -> String {
        self.inner.network().subtree().name().to_owned()
    }

    /// Creates a sub-account for either external keys (receiving addresses) or internal keys (change addresses). This distinction is
    /// a common practice that might help in accounting.
    pub fn chain(&self, change: bool) -> Result<JsBip44PublicSubAccount, JsValue> {
        let account = self.inner.chain(Chain::from(change)).map_err_to_js()?;
        Ok(JsBip44PublicSubAccount::from(account))
    }

    /// Creates a key with a given index used on the chain for storing balance or
    /// authenticating actions. By default these keys are made on the receiving sub-account.
    pub fn key(&self, idx: i32) -> Result<JsBip44PublicKey, JsValue> {
        let sub_account = self.chain(false)?;
        sub_account.key(idx)
    }

    /// The coin identifier of the coin, defined in [SLIP-0044](https://github.com/satoshilabs/slips/blob/master/slip-0044.md).
    #[wasm_bindgen(getter = slip44)]
    pub fn slip44(&self) -> i32 {
        self.inner.bip44_path().parent().slip44()
    }

    /// Accessor for the account index.
    #[wasm_bindgen(getter = account)]
    pub fn account(&self) -> i32 {
        self.inner.bip44_path().account()
    }

    /// Accessor for the BIP32 path of the account.
    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.bip32_path().to_string()
    }

    // Secp specific methods...

    /// Recreates the public API of a BIP44 account from its parts
    #[wasm_bindgen(js_name = fromXpub)]
    pub fn from_xpub(
        account: i32, xpub: &str, network: &str,
    ) -> Result<JsBip44PublicAccount, JsValue> {
        let network = Networks::by_name(network).map_err_to_js()?;
        let inner = Bip44PublicAccount::from_xpub(account, xpub, network).map_err_to_js()?;
        Ok(JsBip44PublicAccount::from(inner))
    }

    /// Returns the extended public key in the BIP32 readable format with the version bytes of the network.
    #[wasm_bindgen(getter = xpub)]
    pub fn to_xpub(&self) -> String {
        self.inner.to_xpub()
    }
}

impl From<Bip44PublicAccount<Secp256k1>> for JsBip44PublicAccount {
    fn from(inner: Bip44PublicAccount<Secp256k1>) -> Self {
        Self { inner }
    }
}

impl Wraps<Bip44PublicAccount<Secp256k1>> for JsBip44PublicAccount {
    fn inner(&self) -> &Bip44PublicAccount<Secp256k1> {
        &self.inner
    }
}

/// Private API for a sub-account of a given account on a given coin that is either used for external keys (receiving addresses) or
/// internal keys (change addresses). Some implementations do not distinguish these and just always use receiving
/// addresses.
#[wasm_bindgen(js_name = Bip44SubAccount)]
#[derive(Clone, Debug)]
pub struct JsBip44SubAccount {
    inner: Bip44SubAccount<Secp256k1>,
}

#[wasm_bindgen(js_class = Bip44SubAccount)]
impl JsBip44SubAccount {
    /// Returns the underlying {@link Bip32Node}.
    pub fn node(&self) -> JsBip32Node {
        JsBip32Node::from(self.inner.node().clone())
    }

    /// Accessor for the name of the underlying network.
    #[wasm_bindgen(getter)]
    pub fn network(&self) -> String {
        self.inner.network().subtree().name().to_owned()
    }

    /// Creates a key with a given index used on the chain for storing balance or
    /// authenticating actions.
    pub fn key(&self, idx: i32) -> Result<JsBip44Key, JsValue> {
        let key = self.inner.key(idx).map_err_to_js()?;
        Ok(JsBip44Key::from(key))
    }

    /// The coin identifier of the coin, defined in [SLIP-0044](https://github.com/satoshilabs/slips/blob/master/slip-0044.md).
    #[wasm_bindgen(getter = slip44)]
    pub fn slip44(&self) -> i32 {
        self.inner.bip44_path().parent().parent().slip44()
    }

    /// Accessor for the account index.
    #[wasm_bindgen(getter = account)]
    pub fn account(&self) -> i32 {
        self.inner.bip44_path().parent().account()
    }

    /// Accessor for whether the sub-account is for change addresses.
    #[wasm_bindgen(getter = change)]
    pub fn change(&self) -> bool {
        self.inner.bip44_path().chain().into()
    }

    /// Accessor for the BIP32 path of the sub-account.
    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.bip32_path().to_string()
    }

    /// Neuters the sub-account and converts it into its public API
    pub fn neuter(&self) -> JsBip44PublicSubAccount {
        let inner = self.inner.neuter();
        JsBip44PublicSubAccount::from(inner)
    }

    // Secp specific methods...

    /// Recreates the private API of a BIP44 sub-account from its parts
    #[wasm_bindgen(js_name = fromXprv)]
    pub fn from_xprv(
        account: i32, change: bool, xprv: &str, network: &str,
    ) -> Result<JsBip44SubAccount, JsValue> {
        let network = Networks::by_name(network).map_err_to_js()?;
        let inner = Bip44SubAccount::from_xprv(account, Chain::from(change), xprv, network)
            .map_err_to_js()?;
        Ok(JsBip44SubAccount::from(inner))
    }

    /// Returns the extended private key in the BIP32 readable format with the version bytes of the network.
    #[wasm_bindgen(getter = xprv)]
    pub fn to_xprv(&self) -> String {
        self.inner.to_xprv()
    }
}

impl From<Bip44SubAccount<Secp256k1>> for JsBip44SubAccount {
    fn from(inner: Bip44SubAccount<Secp256k1>) -> Self {
        Self { inner }
    }
}

impl Wraps<Bip44SubAccount<Secp256k1>> for JsBip44SubAccount {
    fn inner(&self) -> &Bip44SubAccount<Secp256k1> {
        &self.inner
    }
}

/// Public API for a sub-account of a given account on a given coin that is either used for external keys (receiving addresses) or
/// internal keys (change addresses). Some implementations do not distinguish these and just always use receiving
/// addresses.
#[wasm_bindgen(js_name = Bip44PublicSubAccount)]
#[derive(Clone, Debug)]
pub struct JsBip44PublicSubAccount {
    inner: Bip44PublicSubAccount<Secp256k1>,
}

#[wasm_bindgen(js_class = Bip44PublicSubAccount)]
impl JsBip44PublicSubAccount {
    /// Returns the underlying {@link Bip32PublicNode}.
    pub fn node(&self) -> JsBip32PublicNode {
        JsBip32PublicNode::from(self.inner.node().clone())
    }

    /// Accessor for the name of the underlying network.
    #[wasm_bindgen(getter)]
    pub fn network(&self) -> String {
        self.inner.network().subtree().name().to_owned()
    }

    /// Creates a key with a given index used on the chain for storing balance or
    /// authenticating actions.
    pub fn key(&self, idx: i32) -> Result<JsBip44PublicKey, JsValue> {
        let key = self.inner.key(idx).map_err_to_js()?;
        Ok(JsBip44PublicKey::from(key))
    }

    /// The coin identifier of the coin, defined in [SLIP-0044](https://github.com/satoshilabs/slips/blob/master/slip-0044.md).
    #[wasm_bindgen(getter = slip44)]
    pub fn slip44(&self) -> i32 {
        self.inner.bip44_path().parent().parent().slip44()
    }

    /// Accessor for the account index.
    #[wasm_bindgen(getter = account)]
    pub fn account(&self) -> i32 {
        self.inner.bip44_path().parent().account()
    }

    /// Accessor for whether the sub-account is for change addresses.
    #[wasm_bindgen(getter = change)]
    pub fn change(&self) -> bool {
        self.inner.bip44_path().chain().into()
    }

    /// Accessor for the BIP32 path of the sub-account.
    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.bip32_path().to_string()
    }

    // Secp specific methods...

    /// Recreates the public API of a BIP44 sub-account from its parts
    #[wasm_bindgen(js_name = fromXpub)]
    pub fn from_xpub(
        account: i32, change: bool, xpub: &str, network: &str,
    ) -> Result<JsBip44PublicSubAccount, JsValue> {
        let network = Networks::by_name(network).map_err_to_js()?;
        let inner = Bip44PublicSubAccount::from_xpub(account, Chain::from(change), xpub, network)
            .map_err_to_js()?;
        Ok(JsBip44PublicSubAccount::from(inner))
    }

    /// Returns the extended public key in the BIP32 readable format with the version bytes of the network.
    #[wasm_bindgen(getter = xpub)]
    pub fn to_xpub(&self) -> String {
        self.inner.to_xpub()
    }
}

impl From<Bip44PublicSubAccount<Secp256k1>> for JsBip44PublicSubAccount {
    fn from(inner: Bip44PublicSubAccount<Secp256k1>) -> Self {
        Self { inner }
    }
}

impl Wraps<Bip44PublicSubAccount<Secp256k1>> for JsBip44PublicSubAccount {
    fn inner(&self) -> &Bip44PublicSubAccount<Secp256k1> {
        &self.inner
    }
}

/// Represents the private API of a key with a given index within a sub-account used on the chain for storing balance or
/// authenticating actions.
#[wasm_bindgen(js_name = Bip44Key)]
#[derive(Clone, Debug)]
pub struct JsBip44Key {
    inner: Bip44Key<Secp256k1>,
}

#[wasm_bindgen(js_class = Bip44Key)]
impl JsBip44Key {
    /// Returns the underlying {@link Bip32Node}.
    pub fn node(&self) -> JsBip32Node {
        JsBip32Node::from(self.inner.node().clone())
    }

    /// Accessor for the name of the underlying network.
    #[wasm_bindgen(getter)]
    pub fn network(&self) -> String {
        self.inner.network().subtree().name().to_owned()
    }

    /// Creates the private key for authenticating actions.
    #[wasm_bindgen(js_name = privateKey)]
    pub fn to_private_key(&self) -> JsSecpPrivateKey {
        self.node().to_private_key()
    }

    /// The coin identifier of the coin, defined in [SLIP-0044](https://github.com/satoshilabs/slips/blob/master/slip-0044.md).
    #[wasm_bindgen(getter = slip44)]
    pub fn slip44(&self) -> i32 {
        self.inner.bip44_path().parent().parent().parent().slip44()
    }

    /// Accessor for the account index.
    #[wasm_bindgen(getter = account)]
    pub fn account(&self) -> i32 {
        self.inner.bip44_path().parent().parent().account()
    }

    /// Accessor for whether the sub-account is for change addresses.
    #[wasm_bindgen(getter = change)]
    pub fn change(&self) -> bool {
        self.inner.bip44_path().parent().chain().into()
    }

    /// Accessor for the key index within the sub-account.
    #[wasm_bindgen(getter = key)]
    pub fn key(&self) -> i32 {
        self.inner.bip44_path().key()
    }

    /// Accessor for the BIP32 path of the sub-account.
    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.bip32_path().to_string()
    }

    /// Neuters the key and converts it into its public API
    pub fn neuter(&self) -> JsBip44PublicKey {
        let inner = self.inner.neuter();
        JsBip44PublicKey::from(inner)
    }

    // Secp specific methods...

    /// Returns the private key in the Wallet Import Format with the version byte of the network.
    ///
    /// @see SecpPrivateKey.toWif
    #[wasm_bindgen(getter = wif)]
    pub fn to_wif(&self) -> String {
        self.inner.node().to_wif(self.inner.network())
    }
}

impl From<Bip44Key<Secp256k1>> for JsBip44Key {
    fn from(inner: Bip44Key<Secp256k1>) -> Self {
        Self { inner }
    }
}

impl Wraps<Bip44Key<Secp256k1>> for JsBip44Key {
    fn inner(&self) -> &Bip44Key<Secp256k1> {
        &self.inner
    }
}

/// Represents a public key with a given index within a sub-account used on the chain for verifying signatures or validating
/// key identifiers.
#[wasm_bindgen(js_name = Bip44PublicKey)]
#[derive(Clone, Debug)]
pub struct JsBip44PublicKey {
    inner: Bip44PublicKey<Secp256k1>,
}

#[wasm_bindgen(js_class = Bip44PublicKey)]
impl JsBip44PublicKey {
    /// Returns the underlying {@link Bip32PublicNode}.
    pub fn node(&self) -> JsBip32PublicNode {
        JsBip32PublicNode::from(self.inner.node().clone())
    }

    /// Accessor for the name of the underlying network.
    #[wasm_bindgen(getter)]
    pub fn network(&self) -> String {
        self.inner.network().subtree().name().to_owned()
    }

    /// Creates the public key for verifying authentications done by this key.
    #[wasm_bindgen(js_name = publicKey)]
    pub fn to_public_key(&self) -> JsSecpPublicKey {
        self.node().to_public_key()
    }

    /// Creates the key identifier for the public key. This is an extra layer of security for single-use keys, so the
    /// revealing of the public key can be delayed to the point when the authenticated action (spending some coin or
    /// revoking access) makes the public key irrelevant after the action is successful.
    ///
    /// This method chooses the right algorithm used for creating key identifiers on the given network.
    #[wasm_bindgen(js_name = keyId)]
    pub fn to_key_id(&self) -> JsSecpKeyId {
        self.node().to_key_id()
    }

    /// The coin identifier of the coin, defined in [SLIP-0044](https://github.com/satoshilabs/slips/blob/master/slip-0044.md).
    #[wasm_bindgen(getter = slip44)]
    pub fn slip44(&self) -> i32 {
        self.inner.bip44_path().parent().parent().parent().slip44()
    }

    /// Accessor for the account index.
    #[wasm_bindgen(getter = account)]
    pub fn account(&self) -> i32 {
        self.inner.bip44_path().parent().parent().account()
    }

    /// Accessor for whether the sub-account is for change addresses.
    #[wasm_bindgen(getter = change)]
    pub fn change(&self) -> bool {
        self.inner.bip44_path().parent().chain().into()
    }

    /// Accessor for the key index within the sub-account.
    #[wasm_bindgen(getter = key)]
    pub fn key(&self) -> i32 {
        self.inner.bip44_path().key()
    }

    /// Accessor for the BIP32 path of the sub-account.
    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.bip32_path().to_string()
    }

    // Secp specific methods...

    /// Returns the P2PKH address that belongs key with the version byte of the network.
    #[wasm_bindgen(getter = address)]
    pub fn to_p2pkh_addr(&self) -> String {
        self.inner.to_p2pkh_addr()
    }
}

impl From<Bip44PublicKey<Secp256k1>> for JsBip44PublicKey {
    fn from(inner: Bip44PublicKey<Secp256k1>) -> Self {
        Self { inner }
    }
}

impl Wraps<Bip44PublicKey<Secp256k1>> for JsBip44PublicKey {
    fn inner(&self) -> &Bip44PublicKey<Secp256k1> {
        &self.inner
    }
}
