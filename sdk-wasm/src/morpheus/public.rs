use super::*;

#[wasm_bindgen(js_name = MorpheusPublic)]
pub struct JsMorpheusPublic {
    inner: MorpheusPublic,
}

#[wasm_bindgen(js_class = MorpheusPublic)]
impl JsMorpheusPublic {
    pub fn kind(&self, did_kind: &str) -> Result<JsMorpheusPublicKind, JsValue> {
        let did_kind: DidKind = did_kind.parse().map_err_to_js()?;
        self.kind_impl(did_kind)
    }

    #[wasm_bindgen(getter)]
    pub fn personas(&self) -> Result<JsMorpheusPublicKind, JsValue> {
        self.kind_impl(DidKind::Persona)
    }

    #[wasm_bindgen(getter)]
    pub fn devices(&self) -> Result<JsMorpheusPublicKind, JsValue> {
        self.kind_impl(DidKind::Device)
    }

    #[wasm_bindgen(getter)]
    pub fn groups(&self) -> Result<JsMorpheusPublicKind, JsValue> {
        self.kind_impl(DidKind::Group)
    }

    #[wasm_bindgen(getter)]
    pub fn resources(&self) -> Result<JsMorpheusPublicKind, JsValue> {
        self.kind_impl(DidKind::Resource)
    }

    #[wasm_bindgen(js_name = keyById)]
    pub fn key_by_id(&self, id: &JsMKeyId) -> Result<JsMPublicKey, JsValue> {
        let inner = self.inner.key_by_id(id.inner()).map_err_to_js()?;
        Ok(JsMPublicKey::from(inner))
    }

    fn kind_impl(&self, did_kind: DidKind) -> Result<JsMorpheusPublicKind, JsValue> {
        let inner = self.inner.kind(did_kind).map_err_to_js()?;
        Ok(JsMorpheusPublicKind::from(inner))
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
