use super::*;

#[wasm_bindgen(js_name = MorpheusPrivate)]
pub struct JsMorpheusPrivate {
    inner: MorpheusPrivate,
}

#[wasm_bindgen(js_class = MorpheusPrivate)]
impl JsMorpheusPrivate {
    #[wasm_bindgen(getter = pub)]
    pub fn neuter(&self) -> JsMorpheusPublic {
        let inner = self.inner.neuter();
        JsMorpheusPublic::from(inner)
    }

    #[wasm_bindgen(getter)]
    pub fn personas(&self) -> Result<JsMorpheusPrivateKind, JsValue> {
        let inner = self.inner.personas().map_err_to_js()?;
        Ok(JsMorpheusPrivateKind::from(inner))
    }

    #[wasm_bindgen(js_name = keyByPublicKey)]
    pub fn key_by_pk(&self, pk: &JsMPublicKey) -> Result<JsMorpheusPrivateKey, JsValue> {
        let inner = self.inner.key_by_pk(pk.inner()).map_err_to_js()?;
        Ok(JsMorpheusPrivateKey::from(inner))
    }

    #[wasm_bindgen(js_name = keyById)]
    pub fn key_by_id(&self, id: &JsMKeyId) -> Result<JsMorpheusPrivateKey, JsValue> {
        let pk = self.inner.neuter().key_by_id(id.inner()).map_err_to_js()?;
        let js_pk = JsMPublicKey::from(pk);
        self.key_by_pk(&js_pk)
    }

    #[wasm_bindgen(js_name = signDidOperations)]
    pub fn sign_did_operations(
        &self, id: &JsMKeyId, message: &[u8],
    ) -> Result<JsSignedBytes, JsValue> {
        let js_sk = self.key_by_id(id)?;
        let sk: MPrivateKey = js_sk.inner().private_key();
        let sig: MSignature = sk.sign(message);
        let js_pk = JsMPublicKey::from(sk.public_key());
        let js_sig = JsMSignature::from(sig);
        JsSignedBytes::new(&js_pk, message, &js_sig)
    }
}

impl From<MorpheusPrivate> for JsMorpheusPrivate {
    fn from(inner: MorpheusPrivate) -> Self {
        Self { inner }
    }
}

impl Wraps<MorpheusPrivate> for JsMorpheusPrivate {
    fn inner(&self) -> &MorpheusPrivate {
        &self.inner
    }
}
