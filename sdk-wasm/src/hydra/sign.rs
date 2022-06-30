use super::*;

use iop_hydra_sdk::vault::HydraSigner;
use iop_keyvault::secp256k1::SecpPrivateKey;

/// Thin adapter around {@link SecpPrivateKey} for signing Hydra transactions.
#[wasm_bindgen(js_name = HydraSigner)]
pub struct JsHydraSigner {
    inner: SecpPrivateKey,
}

#[wasm_bindgen(js_class = HydraSigner)]
impl JsHydraSigner {
    /// Creates a {@link HydraSigner} from a {@link SecpPrivateKey}.
    #[wasm_bindgen(constructor)]
    pub fn new(inner: JsSecpPrivateKey) -> JsHydraSigner {
        inner.inner().to_owned().into()
    }

    /// Signs the Hydra transaction.
    ///
    /// Fills in signature and id fields, so those can be missing in the unsigned input, but the public key needs to be already
    /// properly set to the one matching the signer private key.
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
