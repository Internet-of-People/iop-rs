use super::*;

#[wasm_bindgen(js_name = MorpheusPrivateKind)]
pub struct JsMorpheusPrivateKind {
    inner: MorpheusPrivateKind,
}

#[wasm_bindgen(js_class = MorpheusPrivateKind)]
impl JsMorpheusPrivateKind {
    // TODO .bip32Path and .network

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        format!("{:?}", self.inner.path())
    }

    #[wasm_bindgen(getter)]
    pub fn count(&self) -> Result<u32, JsValue> {
        self.inner.len().map_err_to_js()
    }

    #[wasm_bindgen(getter = pub)]
    pub fn neuter(&self) -> JsMorpheusPublicKind {
        let inner = self.inner.neuter();
        JsMorpheusPublicKind::from(inner)
    }

    pub fn key(&mut self, idx: i32) -> Result<JsMorpheusPrivateKey, JsValue> {
        let inner = self.inner.key_mut(idx).map_err_to_js()?;
        Ok(JsMorpheusPrivateKey::from(inner))
    }

    pub fn did(&mut self, idx: i32) -> Result<JsDid, JsValue> {
        let sk = self.inner.key_mut(idx).map_err_to_js()?;
        let inner = Did::from(sk.neuter().public_key().key_id());
        Ok(JsDid::from(inner))
    }

    #[wasm_bindgen(js_name = keyByPublicKey)]
    pub fn key_by_pk(&self, id: &JsMPublicKey) -> Result<JsMorpheusPrivateKey, JsValue> {
        let inner = self.inner.key_by_pk(id.inner()).map_err_to_js()?;
        Ok(JsMorpheusPrivateKey::from(inner))
    }
}

impl From<MorpheusPrivateKind> for JsMorpheusPrivateKind {
    fn from(inner: MorpheusPrivateKind) -> Self {
        Self { inner }
    }
}

impl Wraps<MorpheusPrivateKind> for JsMorpheusPrivateKind {
    fn inner(&self) -> &MorpheusPrivateKind {
        &self.inner
    }
}

impl WrapsMut<MorpheusPrivateKind> for JsMorpheusPrivateKind {
    fn inner_mut(&mut self) -> &mut MorpheusPrivateKind {
        &mut self.inner
    }
}
