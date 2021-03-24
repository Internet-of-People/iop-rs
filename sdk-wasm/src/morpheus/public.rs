use super::*;

#[wasm_bindgen(js_name = MorpheusPublic)]
pub struct JsMorpheusPublic {
    inner: MorpheusPublic,
}

#[wasm_bindgen(js_class = MorpheusPublic)]
impl JsMorpheusPublic {
    #[wasm_bindgen(getter)]
    pub fn personas(&self) -> Result<JsMorpheusPublicKind, JsValue> {
        let inner = self.inner.personas().map_err_to_js()?;
        Ok(JsMorpheusPublicKind::from(inner))
    }

    #[wasm_bindgen(js_name = keyById)]
    pub fn key_by_id(&self, id: &JsMKeyId) -> Result<JsMPublicKey, JsValue> {
        let inner = self.inner.key_by_id(id.inner()).map_err_to_js()?;
        Ok(JsMPublicKey::from(inner))
    }
}

impl From<MorpheusPublic> for JsMorpheusPublic {
    fn from(inner: MorpheusPublic) -> Self {
        Self { inner }
    }
}

impl Wraps<MorpheusPublic> for JsMorpheusPublic {
    fn inner(&self) -> &MorpheusPublic {
        &self.inner
    }
}
