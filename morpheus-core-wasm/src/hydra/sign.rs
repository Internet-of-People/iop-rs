use super::*;

use iop_hydra_sdk::vault_hydra::HydraSigner;
use iop_keyvault::secp256k1::SecpPrivateKey;

#[wasm_bindgen(js_name = HydraSigner)]
pub struct JsHydraSigner {
    inner: SecpPrivateKey,
}

#[wasm_bindgen(js_class = HydraSigner)]
impl JsHydraSigner {
    #[wasm_bindgen(constructor)]
    pub fn new(inner: JsSecpPrivateKey) -> JsHydraSigner {
        inner.inner().to_owned().into()
    }

    #[wasm_bindgen(js_name = signHydraTransaction)]
    pub fn sign_hydra_transaction(&self, transaction: &JsValue) -> Result<JsValue, JsValue> {
        let mut tx = transaction.into_serde().map_err_to_js()?;
        self.inner.sign_hydra_transaction(&mut tx).map_err_to_js()?;
        JsValue::from_serde(&tx).map_err_to_js()
    }
}

impl From<SecpPrivateKey> for JsHydraSigner {
    fn from(inner: SecpPrivateKey) -> Self {
        Self { inner }
    }
}

impl Wraps<SecpPrivateKey> for JsHydraSigner {
    fn inner(&self) -> &SecpPrivateKey {
        &self.inner
    }
}
