use super::*;

#[wasm_bindgen(js_name = HydraPublic)]
pub struct JsHydraPublic {
    inner: HydraPublic,
}

#[wasm_bindgen(js_class = HydraPublic)]
impl JsHydraPublic {
    pub fn key(&mut self, idx: i32) -> Result<JsBip44PublicKey, JsValue> {
        let inner = self.inner.key(idx).map_err_to_js()?;
        Ok(JsBip44PublicKey::from(inner))
    }

    #[wasm_bindgen(getter)]
    pub fn xpub(&self) -> Result<String, JsValue> {
        let res = self.inner().xpub().map_err_to_js()?;
        Ok(res)
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

    #[wasm_bindgen(js_name = keyByAddress)]
    pub fn key_by_p2pkh_addr(&self, addr: &str) -> Result<JsBip44PublicKey, JsValue> {
        let inner = self.inner.key_by_p2pkh_addr(addr).map_err_to_js()?;
        Ok(JsBip44PublicKey::from(inner))
    }
}

impl From<HydraPublic> for JsHydraPublic {
    fn from(inner: HydraPublic) -> Self {
        Self { inner }
    }
}

impl Wraps<HydraPublic> for JsHydraPublic {
    fn inner(&self) -> &HydraPublic {
        &self.inner
    }
}
