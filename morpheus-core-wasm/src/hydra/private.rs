use super::*;

#[wasm_bindgen(js_name = HydraPrivate)]
pub struct JsHydraPrivate {
    inner: HydraPrivate,
}

#[wasm_bindgen(js_class = HydraPrivate)]
impl JsHydraPrivate {
    #[wasm_bindgen(getter = pub)]
    pub fn neuter(&self) -> JsHydraPublic {
        let inner = self.inner.neuter();
        JsHydraPublic::from(inner)
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
