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

    // #[wasm_bindgen(js_name = keyById)]
    // pub fn key_by_id(&self, id: &JsSecpKeyId) -> Result<JsSecpPublicKey, JsValue> {
    //     let inner = self.inner.key_by_id(id.inner()).map_err_to_js()?;
    //     Ok(JsSecpPublicKey::from(inner))
    // }
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
