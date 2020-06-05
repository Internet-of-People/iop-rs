use iop_keyvault::{
    secp256k1::Secp256k1, Bip44, Bip44Account, Bip44Coin, Bip44Key, Bip44PublicAccount,
    Bip44PublicKey, Bip44PublicSubAccount, Bip44SubAccount, Chain,
};
use wasm_bindgen::prelude::*;

use super::*;

#[wasm_bindgen(js_name = Bip44)]
#[derive(Clone, Debug)]
pub struct JsBip44;

#[wasm_bindgen(js_class = Bip44)]
impl JsBip44 {
    pub fn network(seed: &JsSeed, name: &str) -> Result<JsBip44Coin, JsValue> {
        let network = Networks::by_name(name).map_err(err_to_js)?;
        let coin = Bip44.network(&seed.inner(), network).map_err(err_to_js)?;
        Ok(JsBip44Coin::from(coin))
    }
}

#[wasm_bindgen(js_name = Bip44Coin)]
#[derive(Clone, Debug)]
pub struct JsBip44Coin {
    inner: Bip44Coin<Secp256k1>,
}

#[wasm_bindgen(js_class = Bip44Coin)]
impl JsBip44Coin {
    pub fn node(&self) -> JsBip32Node {
        JsBip32Node::from(self.inner.node().clone())
    }

    pub fn account(&self, account: i32) -> Result<JsBip44Account, JsValue> {
        let account = self.inner.account(account).map_err(err_to_js)?;
        Ok(JsBip44Account::from(account))
    }

    #[wasm_bindgen(getter = slip44)]
    pub fn slip44(&self) -> i32 {
        self.inner.bip44_path().slip44()
    }

    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.bip32_path().to_string()
    }

    // Secp specific methods...

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

#[wasm_bindgen(js_name = Bip44Account)]
#[derive(Clone, Debug)]
pub struct JsBip44Account {
    inner: Bip44Account<Secp256k1>,
}

#[wasm_bindgen(js_class = Bip44Account)]
impl JsBip44Account {
    pub fn node(&self) -> JsBip32Node {
        JsBip32Node::from(self.inner.node().clone())
    }

    pub fn chain(&self, change: bool) -> Result<JsBip44SubAccount, JsValue> {
        let account = self.inner.chain(Chain::from(change)).map_err(err_to_js)?;
        Ok(JsBip44SubAccount::from(account))
    }

    pub fn key(&self, idx: i32) -> Result<JsBip44Key, JsValue> {
        let sub_account = self.chain(false)?;
        sub_account.key(idx)
    }

    #[wasm_bindgen(getter = slip44)]
    pub fn slip44(&self) -> i32 {
        self.inner.bip44_path().parent().slip44()
    }

    #[wasm_bindgen(getter = account)]
    pub fn account(&self) -> i32 {
        self.inner.bip44_path().account()
    }

    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.bip32_path().to_string()
    }

    pub fn neuter(&self) -> JsBip44PublicAccount {
        let inner = self.inner.neuter();
        JsBip44PublicAccount::from(inner)
    }

    // Secp specific methods...

    #[wasm_bindgen(js_name = fromXprv)]
    pub fn from_xprv(account: i32, xprv: &str, network: &str) -> Result<JsBip44Account, JsValue> {
        let network = Networks::by_name(network).map_err(err_to_js)?;
        let inner = Bip44Account::from_xprv(account, xprv, network).map_err(err_to_js)?;
        Ok(JsBip44Account::from(inner))
    }

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

#[wasm_bindgen(js_name = Bip44PublicAccount)]
#[derive(Clone, Debug)]
pub struct JsBip44PublicAccount {
    inner: Bip44PublicAccount<Secp256k1>,
}

#[wasm_bindgen(js_class = Bip44PublicAccount)]
impl JsBip44PublicAccount {
    pub fn node(&self) -> JsBip32PublicNode {
        JsBip32PublicNode::from(self.inner.node().clone())
    }

    pub fn chain(&self, change: bool) -> Result<JsBip44PublicSubAccount, JsValue> {
        let account = self.inner.chain(Chain::from(change)).map_err(err_to_js)?;
        Ok(JsBip44PublicSubAccount::from(account))
    }

    pub fn key(&self, idx: i32) -> Result<JsBip44PublicKey, JsValue> {
        let sub_account = self.chain(false)?;
        sub_account.key(idx)
    }

    #[wasm_bindgen(getter = slip44)]
    pub fn slip44(&self) -> i32 {
        self.inner.bip44_path().parent().slip44()
    }

    #[wasm_bindgen(getter = account)]
    pub fn account(&self) -> i32 {
        self.inner.bip44_path().account()
    }

    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.bip32_path().to_string()
    }

    // Secp specific methods...

    #[wasm_bindgen(js_name = fromXpub)]
    pub fn from_xpub(
        account: i32, xpub: &str, network: &str,
    ) -> Result<JsBip44PublicAccount, JsValue> {
        let network = Networks::by_name(network).map_err(err_to_js)?;
        let inner = Bip44PublicAccount::from_xpub(account, xpub, network).map_err(err_to_js)?;
        Ok(JsBip44PublicAccount::from(inner))
    }

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

#[wasm_bindgen(js_name = Bip44SubAccount)]
#[derive(Clone, Debug)]
pub struct JsBip44SubAccount {
    inner: Bip44SubAccount<Secp256k1>,
}

#[wasm_bindgen(js_class = Bip44SubAccount)]
impl JsBip44SubAccount {
    pub fn node(&self) -> JsBip32Node {
        JsBip32Node::from(self.inner.node().clone())
    }

    pub fn key(&self, idx: i32) -> Result<JsBip44Key, JsValue> {
        let key = self.inner.key(idx).map_err(err_to_js)?;
        Ok(JsBip44Key::from(key))
    }

    #[wasm_bindgen(getter = slip44)]
    pub fn slip44(&self) -> i32 {
        self.inner.bip44_path().parent().parent().slip44()
    }

    #[wasm_bindgen(getter = account)]
    pub fn account(&self) -> i32 {
        self.inner.bip44_path().parent().account()
    }

    #[wasm_bindgen(getter = change)]
    pub fn change(&self) -> bool {
        self.inner.bip44_path().chain().into()
    }

    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.bip32_path().to_string()
    }

    pub fn neuter(&self) -> JsBip44PublicSubAccount {
        let inner = self.inner.neuter();
        JsBip44PublicSubAccount::from(inner)
    }

    // Secp specific methods...

    #[wasm_bindgen(js_name = fromXprv)]
    pub fn from_xprv(
        account: i32, change: bool, xprv: &str, network: &str,
    ) -> Result<JsBip44SubAccount, JsValue> {
        let network = Networks::by_name(network).map_err(err_to_js)?;
        let inner = Bip44SubAccount::from_xprv(account, Chain::from(change), xprv, network)
            .map_err(err_to_js)?;
        Ok(JsBip44SubAccount::from(inner))
    }

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

#[wasm_bindgen(js_name = Bip44PublicSubAccount)]
#[derive(Clone, Debug)]
pub struct JsBip44PublicSubAccount {
    inner: Bip44PublicSubAccount<Secp256k1>,
}

#[wasm_bindgen(js_class = Bip44PublicSubAccount)]
impl JsBip44PublicSubAccount {
    pub fn node(&self) -> JsBip32PublicNode {
        JsBip32PublicNode::from(self.inner.node().clone())
    }

    pub fn key(&self, idx: i32) -> Result<JsBip44PublicKey, JsValue> {
        let key = self.inner.key(idx).map_err(err_to_js)?;
        Ok(JsBip44PublicKey::from(key))
    }

    #[wasm_bindgen(getter = slip44)]
    pub fn slip44(&self) -> i32 {
        self.inner.bip44_path().parent().parent().slip44()
    }

    #[wasm_bindgen(getter = account)]
    pub fn account(&self) -> i32 {
        self.inner.bip44_path().parent().account()
    }

    #[wasm_bindgen(getter = change)]
    pub fn change(&self) -> bool {
        self.inner.bip44_path().chain().into()
    }

    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.bip32_path().to_string()
    }

    // Secp specific methods...

    #[wasm_bindgen(js_name = fromXpub)]
    pub fn from_xpub(
        account: i32, change: bool, xpub: &str, network: &str,
    ) -> Result<JsBip44PublicSubAccount, JsValue> {
        let network = Networks::by_name(network).map_err(err_to_js)?;
        let inner = Bip44PublicSubAccount::from_xpub(account, Chain::from(change), xpub, network)
            .map_err(err_to_js)?;
        Ok(JsBip44PublicSubAccount::from(inner))
    }

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

#[wasm_bindgen(js_name = Bip44Key)]
#[derive(Clone, Debug)]
pub struct JsBip44Key {
    inner: Bip44Key<Secp256k1>,
}

#[wasm_bindgen(js_class = Bip44Key)]
impl JsBip44Key {
    pub fn node(&self) -> JsBip32Node {
        JsBip32Node::from(self.inner.node().clone())
    }

    #[wasm_bindgen(js_name = privateKey)]
    pub fn to_private_key(&self) -> JsSecpPrivateKey {
        self.node().to_private_key()
    }

    #[wasm_bindgen(getter = slip44)]
    pub fn slip44(&self) -> i32 {
        self.inner.bip44_path().parent().parent().parent().slip44()
    }

    #[wasm_bindgen(getter = account)]
    pub fn account(&self) -> i32 {
        self.inner.bip44_path().parent().parent().account()
    }

    #[wasm_bindgen(getter = change)]
    pub fn change(&self) -> bool {
        self.inner.bip44_path().parent().chain().into()
    }

    #[wasm_bindgen(getter = key)]
    pub fn key(&self) -> i32 {
        self.inner.bip44_path().key()
    }

    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.bip32_path().to_string()
    }

    pub fn neuter(&self) -> JsBip44PublicKey {
        let inner = self.inner.neuter();
        JsBip44PublicKey::from(inner)
    }

    // Secp specific methods...

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

#[wasm_bindgen(js_name = Bip44PublicKey)]
#[derive(Clone, Debug)]
pub struct JsBip44PublicKey {
    inner: Bip44PublicKey<Secp256k1>,
}

#[wasm_bindgen(js_class = Bip44PublicKey)]
impl JsBip44PublicKey {
    pub fn node(&self) -> JsBip32PublicNode {
        JsBip32PublicNode::from(self.inner.node().clone())
    }

    #[wasm_bindgen(js_name = publicKey)]
    pub fn to_public_key(&self) -> JsSecpPublicKey {
        self.node().to_public_key()
    }

    #[wasm_bindgen(js_name = keyId)]
    pub fn to_key_id(&self) -> JsSecpKeyId {
        self.node().to_key_id()
    }

    #[wasm_bindgen(getter = slip44)]
    pub fn slip44(&self) -> i32 {
        self.inner.bip44_path().parent().parent().parent().slip44()
    }

    #[wasm_bindgen(getter = account)]
    pub fn account(&self) -> i32 {
        self.inner.bip44_path().parent().parent().account()
    }

    #[wasm_bindgen(getter = change)]
    pub fn change(&self) -> bool {
        self.inner.bip44_path().parent().chain().into()
    }

    #[wasm_bindgen(getter = key)]
    pub fn key(&self) -> i32 {
        self.inner.bip44_path().key()
    }

    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.bip32_path().to_string()
    }

    // Secp specific methods...

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
