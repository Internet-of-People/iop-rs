use iop_keyvault::{
    multicipher::MPrivateKey,
    secp256k1::{Bip178, SecpPrivateKey},
    PrivateKey,
};
use wasm_bindgen::prelude::*;

use super::*;

#[wasm_bindgen(js_name = PrivateKey)]
#[derive(Clone, Debug)]
pub struct JsMPrivateKey {
    inner: MPrivateKey,
}

#[wasm_bindgen(js_class = PrivateKey)]
impl JsMPrivateKey {
    #[wasm_bindgen(js_name = fromSecp)]
    pub fn from_secp(sk: &JsSecpPrivateKey) -> Self {
        let inner = MPrivateKey::from(sk.inner().clone());
        Self { inner }
    }

    #[wasm_bindgen(js_name = publicKey)]
    pub fn public_key(&self) -> JsMPublicKey {
        JsMPublicKey::from(self.inner.public_key())
    }

    #[wasm_bindgen(js_name = signEcdsa)]
    pub fn sign_ecdsa(&self, data: &[u8]) -> JsMSignature {
        let sig = self.inner.sign(data);
        JsMSignature::from(sig)
    }
}

impl From<MPrivateKey> for JsMPrivateKey {
    fn from(inner: MPrivateKey) -> Self {
        Self { inner }
    }
}

impl Wraps<MPrivateKey> for JsMPrivateKey {
    fn inner(&self) -> &MPrivateKey {
        &self.inner
    }
}

#[wasm_bindgen(js_name = SecpPrivateKey)]
#[derive(Clone, Debug)]
pub struct JsSecpPrivateKey {
    inner: SecpPrivateKey,
}

#[wasm_bindgen(js_class = SecpPrivateKey)]
impl JsSecpPrivateKey {
    #[wasm_bindgen(js_name = fromArkPassphrase)]
    pub fn from_ark_passphrase(phrase: &str) -> Result<JsSecpPrivateKey, JsValue> {
        let inner = SecpPrivateKey::from_ark_passphrase(phrase).map_err(err_to_js)?;
        Ok(Self { inner })
    }

    #[wasm_bindgen(js_name = toWif)]
    pub fn to_wif(&self, network: &str) -> Result<String, JsValue> {
        let network = Networks::by_name(network).map_err(err_to_js)?;
        Ok(self.inner.to_wif(network.wif(), Bip178::Compressed))
    }

    #[wasm_bindgen(js_name = publicKey)]
    pub fn public_key(&self) -> JsSecpPublicKey {
        JsSecpPublicKey::from(self.inner.public_key())
    }

    #[wasm_bindgen(js_name = signEcdsa)]
    pub fn sign_ecdsa(&self, data: &[u8]) -> JsSecpSignature {
        let sig = self.inner.sign(data);
        JsSecpSignature::from(sig)
    }
}

impl From<SecpPrivateKey> for JsSecpPrivateKey {
    fn from(inner: SecpPrivateKey) -> Self {
        Self { inner }
    }
}

impl Wraps<SecpPrivateKey> for JsSecpPrivateKey {
    fn inner(&self) -> &SecpPrivateKey {
        &self.inner
    }
}
