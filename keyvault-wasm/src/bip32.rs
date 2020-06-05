use iop_keyvault::{secp256k1::Secp256k1, Bip32, Bip32Node, Bip32PublicNode};
use wasm_bindgen::prelude::*;

use super::*;

#[wasm_bindgen(js_name = Bip32)]
#[derive(Clone, Debug)]
pub struct JsBip32;

#[wasm_bindgen(js_name = Bip32)]
impl JsBip32 {
    pub fn master(seed: &JsSeed, name: &str) -> Result<JsBip32Node, JsValue> {
        let network = Networks::by_name(name).map_err(err_to_js)?;
        let node = Bip32.master(&seed.inner(), network.subtree());
        Ok(JsBip32Node::from(node))
    }
}

#[wasm_bindgen(js_name = Bip32Node)]
#[derive(Clone, Debug)]
pub struct JsBip32Node {
    inner: Bip32Node<Secp256k1>,
}

#[wasm_bindgen(js_class = Bip32Node)]
impl JsBip32Node {
    #[wasm_bindgen(getter)]
    pub fn path(&self) -> String {
        self.inner.path().to_string()
    }

    #[wasm_bindgen(js_name = deriveNormal)]
    pub fn derive_normal(&self, idx: i32) -> Result<JsBip32Node, JsValue> {
        let child = self.inner.derive_normal(idx).map_err(err_to_js)?;
        Ok(JsBip32Node::from(child))
    }

    #[wasm_bindgen(js_name = deriveHardened)]
    pub fn derive_hardened(&self, idx: i32) -> Result<JsBip32Node, JsValue> {
        let child = self.inner.derive_hardened(idx).map_err(err_to_js)?;
        Ok(JsBip32Node::from(child))
    }

    #[wasm_bindgen(js_name = privateKey)]
    pub fn to_private_key(&self) -> JsSecpPrivateKey {
        JsSecpPrivateKey::from(self.inner.private_key())
    }

    #[wasm_bindgen(js_name = neuter)]
    pub fn neuter(&self) -> JsBip32PublicNode {
        let inner = self.inner.neuter();
        JsBip32PublicNode::from(inner)
    }

    // Secp specific methods...

    #[wasm_bindgen(js_name = toXprv)]
    pub fn to_xprv(&self, name: &str) -> Result<String, JsValue> {
        let network = Networks::by_name(name).map_err(err_to_js)?;
        Ok(self.inner.to_xprv(network))
    }

    #[wasm_bindgen(js_name = toWif)]
    pub fn to_wif(&self, name: &str) -> Result<String, JsValue> {
        let network = Networks::by_name(name).map_err(err_to_js)?;
        Ok(self.inner.to_wif(network))
    }
}

impl From<Bip32Node<Secp256k1>> for JsBip32Node {
    fn from(inner: Bip32Node<Secp256k1>) -> Self {
        Self { inner }
    }
}

impl Wraps<Bip32Node<Secp256k1>> for JsBip32Node {
    fn inner(&self) -> &Bip32Node<Secp256k1> {
        &self.inner
    }
}

#[wasm_bindgen(js_name = Bip32PublicNode)]
#[derive(Clone, Debug)]
pub struct JsBip32PublicNode {
    inner: Bip32PublicNode<Secp256k1>,
}

#[wasm_bindgen(js_class = Bip32PublicNode)]
impl JsBip32PublicNode {
    #[wasm_bindgen(getter)]
    pub fn path(&self) -> String {
        self.inner.path().to_string()
    }

    #[wasm_bindgen(js_name = deriveNormal)]
    pub fn derive_normal(&self, idx: i32) -> Result<JsBip32PublicNode, JsValue> {
        let child = self.inner.derive_normal(idx).map_err(err_to_js)?;
        Ok(JsBip32PublicNode::from(child))
    }

    #[wasm_bindgen(js_name = publicKey)]
    pub fn to_public_key(&self) -> JsSecpPublicKey {
        JsSecpPublicKey::from(self.inner.public_key())
    }

    #[wasm_bindgen(js_name = keyId)]
    pub fn to_key_id(&self) -> JsSecpKeyId {
        JsSecpKeyId::from(self.inner.key_id())
    }

    // Secp specific methods...

    #[wasm_bindgen(js_name = toXpub)]
    pub fn to_xpub(&self, name: &str) -> Result<String, JsValue> {
        let network = Networks::by_name(name).map_err(err_to_js)?;
        Ok(self.inner.to_xpub(network))
    }

    #[wasm_bindgen(js_name = toP2pkh)]
    pub fn to_p2pkh_addr(&self, name: &str) -> Result<String, JsValue> {
        let network = Networks::by_name(name).map_err(err_to_js)?;
        Ok(self.inner.to_p2pkh_addr(network))
    }
}

impl From<Bip32PublicNode<Secp256k1>> for JsBip32PublicNode {
    fn from(inner: Bip32PublicNode<Secp256k1>) -> Self {
        Self { inner }
    }
}

impl Wraps<Bip32PublicNode<Secp256k1>> for JsBip32PublicNode {
    fn inner(&self) -> &Bip32PublicNode<Secp256k1> {
        &self.inner
    }
}
