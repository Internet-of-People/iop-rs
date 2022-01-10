use super::*;

#[wasm_bindgen(js_name = PublicKey)]
#[derive(Clone, Debug)]
pub struct JsMPublicKey {
    inner: MPublicKey,
}

#[wasm_bindgen(js_class = PublicKey)]
impl JsMPublicKey {
    #[wasm_bindgen(constructor)]
    pub fn new(pub_key_str: &str) -> Result<JsMPublicKey, JsValue> {
        let inner: MPublicKey = pub_key_str.parse().map_err_to_js()?;
        Ok(Self { inner })
    }

    #[wasm_bindgen(js_name = fromSecp)]
    pub fn from_secp(pk: &JsSecpPublicKey) -> Self {
        let inner = MPublicKey::from(pk.inner().clone());
        Self { inner }
    }

    #[wasm_bindgen]
    pub fn prefix() -> String {
        MPublicKey::PREFIX.to_string()
    }

    #[wasm_bindgen(js_name = keyId)]
    pub fn key_id(&self) -> JsMKeyId {
        JsMKeyId::from(self.inner.key_id())
    }

    #[wasm_bindgen(js_name = validateId)]
    pub fn validate_id(&self, key_id: &JsMKeyId) -> bool {
        self.inner.validate_id(key_id.inner())
    }

    #[wasm_bindgen(js_name = validateEcdsa)]
    pub fn validate_ecdsa(&self, data: &[u8], signature: &JsMSignature) -> bool {
        self.inner.verify(data, signature.inner())
    }

    // Note that Clippy complains if you call these methods to_string. But implementing Display is not enough to get a toString in JS.
    #[wasm_bindgen(js_name=toString)]
    pub fn stringify(&self) -> String {
        self.inner.to_string()
    }
}

impl From<MPublicKey> for JsMPublicKey {
    fn from(inner: MPublicKey) -> Self {
        Self { inner }
    }
}

impl Wraps<MPublicKey> for JsMPublicKey {
    fn inner(&self) -> &MPublicKey {
        &self.inner
    }
}

#[wasm_bindgen(js_name = SecpPublicKey)]
#[derive(Clone, Debug)]
pub struct JsSecpPublicKey {
    inner: SecpPublicKey,
}

#[wasm_bindgen(js_class = SecpPublicKey)]
impl JsSecpPublicKey {
    #[wasm_bindgen(constructor)]
    pub fn new(key: &str) -> Result<JsSecpPublicKey, JsValue> {
        let inner: SecpPublicKey = key.parse().map_err_to_js()?;
        Ok(Self { inner })
    }

    #[wasm_bindgen(js_name = keyId)]
    pub fn key_id(&self) -> JsSecpKeyId {
        JsSecpKeyId::from(self.inner.key_id())
    }

    #[wasm_bindgen(js_name = arkKeyId)]
    pub fn ark_key_id(&self) -> JsSecpKeyId {
        JsSecpKeyId::from(self.inner.ark_key_id())
    }

    #[wasm_bindgen(js_name = validateId)]
    pub fn validate_id(&self, key_id: &JsSecpKeyId) -> bool {
        self.inner.validate_id(key_id.inner())
    }

    #[wasm_bindgen(js_name = validateArkId)]
    pub fn validate_ark_id(&self, key_id: &JsSecpKeyId) -> bool {
        self.inner.validate_ark_id(key_id.inner())
    }

    #[wasm_bindgen(js_name = validateEcdsa)]
    pub fn validate_ecdsa(&self, data: &[u8], signature: &JsSecpSignature) -> bool {
        self.inner.verify(data, signature.inner())
    }

    // Note that Clippy complains if you call these methods to_string. But implementing Display is not enough to get a toString in JS.
    #[wasm_bindgen(js_name=toString)]
    pub fn stringify(&self) -> String {
        self.inner.to_string()
    }
}

impl From<SecpPublicKey> for JsSecpPublicKey {
    fn from(inner: SecpPublicKey) -> Self {
        Self { inner }
    }
}

impl Wraps<SecpPublicKey> for JsSecpPublicKey {
    fn inner(&self) -> &SecpPublicKey {
        &self.inner
    }
}
