use super::*;

#[wasm_bindgen(js_name = MorpheusPublicKind)]
pub struct JsMorpheusPublicKind {
    inner: MorpheusPublicKind,
}

#[wasm_bindgen(js_class = MorpheusPublicKind)]
impl JsMorpheusPublicKind {
    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        format!("{:?}", self.inner.path())
    }

    #[wasm_bindgen(getter)]
    pub fn count(&self) -> Result<u32, JsValue> {
        self.inner.len().map_err_to_js().map(|c| c as u32)
    }

    pub fn key(&self, idx: i32) -> Result<JsMPublicKey, JsValue> {
        let inner = self.inner.key(idx).map_err_to_js()?;
        Ok(JsMPublicKey::from(inner))
    }

    pub fn did(&self, idx: i32) -> Result<JsDid, JsValue> {
        let pk = self.inner.key(idx).map_err_to_js()?;
        let inner = Did::from(pk.key_id());
        Ok(JsDid::from(inner))
    }

    #[wasm_bindgen(js_name = keyById)]
    pub fn key_by_id(&self, id: &JsMKeyId) -> Result<JsMPublicKey, JsValue> {
        let inner = self.inner.key_by_id(id.inner()).map_err_to_js()?;
        Ok(JsMPublicKey::from(inner))
    }
}

impl From<MorpheusPublicKind> for JsMorpheusPublicKind {
    fn from(inner: MorpheusPublicKind) -> Self {
        Self { inner }
    }
}

impl Wraps<MorpheusPublicKind> for JsMorpheusPublicKind {
    fn inner(&self) -> &MorpheusPublicKind {
        &self.inner
    }
}
