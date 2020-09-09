use super::*;

#[wasm_bindgen(js_name = HydraPrivate)]
pub struct JsHydraPrivate {
    inner: HydraPrivate,
}

#[wasm_bindgen(js_class = HydraPrivate)]
impl JsHydraPrivate {
    #[wasm_bindgen(getter = pub)]
    pub fn public(&self) -> JsHydraPublic {
        let inner = self.inner.public();
        JsHydraPublic::from(inner)
    }

    #[wasm_bindgen(getter)]
    pub fn network(&self) -> String {
        self.inner.network().subtree().name().to_owned()
    }

    pub fn key(&mut self, idx: i32) -> Result<JsBip44Key, JsValue> {
        let inner = self.inner.key(idx).map_err_to_js()?;
        Ok(JsBip44Key::from(inner))
    }

    #[wasm_bindgen(js_name = keyByPublicKey)]
    pub fn key_by_pk(&self, id: &JsSecpPublicKey) -> Result<JsBip44Key, JsValue> {
        let inner = self.inner.key_by_pk(id.inner()).map_err_to_js()?;
        Ok(JsBip44Key::from(inner))
    }

    #[wasm_bindgen(getter)]
    pub fn xpub(&self) -> Result<String, JsValue> {
        let res = self.inner().xpub().map_err_to_js()?;
        Ok(res)
    }

    #[wasm_bindgen(getter)]
    pub fn xprv(&self) -> String {
        self.inner().xprv()
    }

    #[wasm_bindgen(getter = receiveKeys)]
    pub fn receive_keys(&self) -> Result<u32, JsValue> {
        let res = self.inner().receive_keys().map_err_to_js()?;
        Ok(res)
    }

    #[wasm_bindgen(getter = changeKeys)]
    pub fn change_keys(&self) -> Result<u32, JsValue> {
        let res = self.inner().change_keys().map_err_to_js()?;
        Ok(res)
    }

    #[wasm_bindgen(js_name = signHydraTransaction)]
    pub fn sign_hydra_transaction(&self, hyd_addr: &str, tx: &JsValue) -> Result<JsValue, JsValue> {
        let mut tx: HydraTransactionData =
            tx.into_serde().with_context(|| "Parsing ITransactionData").map_err_to_js()?;
        self.inner
            .sign_hydra_transaction(hyd_addr, &mut tx)
            .context("Signing ITransactionData")
            .map_err_to_js()?;
        let signed_tx =
            JsValue::from_serde(&tx).context("Serializing ITransactionData").map_err_to_js()?;
        Ok(signed_tx)
    }
}

impl From<HydraPrivate> for JsHydraPrivate {
    fn from(inner: HydraPrivate) -> Self {
        Self { inner }
    }
}

impl Wraps<HydraPrivate> for JsHydraPrivate {
    fn inner(&self) -> &HydraPrivate {
        &self.inner
    }
}
