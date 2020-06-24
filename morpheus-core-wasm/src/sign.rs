use super::*;

#[wasm_bindgen(js_name = SignedBytes)]
pub struct JsSignedBytes {
    inner: Signed<Box<[u8]>>,
}

#[wasm_bindgen(js_class = SignedBytes)]
impl JsSignedBytes {
    #[wasm_bindgen(constructor)]
    pub fn new(
        public_key: &JsMPublicKey, content: &[u8], signature: &JsMSignature,
    ) -> Result<JsSignedBytes, JsValue> {
        let inner = Signed::new(
            public_key.inner().to_owned(),
            content.to_owned().into_boxed_slice(),
            signature.inner().to_owned(),
        );
        Ok(Self { inner })
    }

    #[wasm_bindgen(getter, js_name = publicKey)]
    pub fn public_key(&self) -> JsMPublicKey {
        JsMPublicKey::from(self.inner.public_key().to_owned())
    }

    #[wasm_bindgen(getter)]
    pub fn content(&self) -> Box<[u8]> {
        self.inner.content().clone()
    }

    #[wasm_bindgen(getter)]
    pub fn signature(&self) -> JsMSignature {
        JsMSignature::from(self.inner.signature().to_owned())
    }

    pub fn validate(&self) -> Result<bool, JsValue> {
        let content = self.inner.content().content_to_sign().map_err_to_js()?;
        Ok(self.inner.public_key().verify(content, &self.inner.signature()))
    }
}

impl From<Signed<Box<[u8]>>> for JsSignedBytes {
    fn from(inner: Signed<Box<[u8]>>) -> Self {
        Self { inner }
    }
}

impl Wraps<Signed<Box<[u8]>>> for JsSignedBytes {
    fn inner(&self) -> &Signed<Box<[u8]>> {
        &self.inner
    }
}

#[wasm_bindgen(js_name = SignedJson)]
pub struct JsSignedJson {
    inner: Signed<serde_json::Value>,
}

#[wasm_bindgen(js_class = SignedJson)]
impl JsSignedJson {
    #[wasm_bindgen(constructor)]
    pub fn new(
        public_key: &JsMPublicKey, content: &JsValue, signature: &JsMSignature,
    ) -> Result<JsSignedJson, JsValue> {
        let inner = Signed::new(
            public_key.inner().to_owned(),
            content.into_serde().map_err_to_js()?,
            signature.inner().to_owned(),
        );
        Ok(Self { inner })
    }

    #[wasm_bindgen(getter, js_name = publicKey)]
    pub fn public_key(&self) -> JsMPublicKey {
        JsMPublicKey::from(self.inner.public_key().to_owned())
    }

    #[wasm_bindgen(getter)]
    pub fn content(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(self.inner.content()).map_err_to_js()
    }

    #[wasm_bindgen(getter)]
    pub fn signature(&self) -> JsMSignature {
        JsMSignature::from(self.inner.signature().to_owned())
    }

    pub fn validate(&self) -> bool {
        match self.inner.content().content_to_sign() {
            Ok(content) => self.inner.public_key().verify(content, &self.inner.signature()),
            Err(_) => false,
        }
    }

    #[wasm_bindgen(js_name = validateWithKeyId)]
    pub fn validate_with_keyid(&self, signer_id: &JsMKeyId) -> bool {
        self.inner.public_key().validate_id(&signer_id.inner()) && self.validate()
    }

    // TODO return red/yellow/green instead of bool
    #[wasm_bindgen(js_name = validateWithDidDoc)]
    pub fn validate_with_did_doc(
        &self, did_doc_str: &str, from_height_inc: Option<BlockHeight>,
        until_height_exc: Option<BlockHeight>,
    ) -> Result<JsValue, JsValue> {
        let did_doc = serde_json::from_str(did_doc_str).map_err_to_js()?;
        let result = self
            .inner
            .validate_with_did_doc(&did_doc, from_height_inc, until_height_exc)
            .map_err_to_js()?;
        Ok(JsValidationResult { inner: result }.into())
    }
}

impl From<Signed<serde_json::Value>> for JsSignedJson {
    fn from(inner: Signed<Value>) -> Self {
        Self { inner }
    }
}

impl Wraps<Signed<serde_json::Value>> for JsSignedJson {
    fn inner(&self) -> &Signed<Value> {
        &self.inner
    }
}

#[wasm_bindgen(js_name = ValidationIssue)]
pub struct JsValidationIssue {
    inner: ValidationIssue,
}

#[wasm_bindgen(js_class = ValidationIssue)]
impl JsValidationIssue {
    #[wasm_bindgen(getter)]
    pub fn code(&self) -> u32 {
        self.inner.code()
    }

    #[wasm_bindgen(getter)]
    pub fn severity(&self) -> String {
        self.inner.severity().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn reason(&self) -> String {
        self.inner.reason().to_string()
    }
}

impl From<ValidationIssue> for JsValidationIssue {
    fn from(inner: ValidationIssue) -> Self {
        Self { inner }
    }
}

impl Wraps<ValidationIssue> for JsValidationIssue {
    fn inner(&self) -> &ValidationIssue {
        &self.inner
    }
}

#[wasm_bindgen(js_name = ValidationResult)]
pub struct JsValidationResult {
    inner: ValidationResult,
}

#[wasm_bindgen(js_class = ValidationResult)]
impl JsValidationResult {
    #[wasm_bindgen(getter)]
    pub fn status(&self) -> String {
        self.inner.status().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn messages(&self) -> Box<[JsValue]> {
        let msgs = self
            .inner
            .issues()
            .iter()
            .map(|issue| JsValidationIssue { inner: issue.to_owned() }.into())
            .collect::<Vec<_>>();
        msgs.into_boxed_slice()
    }
}

impl From<ValidationResult> for JsValidationResult {
    fn from(inner: ValidationResult) -> Self {
        Self { inner }
    }
}

impl Wraps<ValidationResult> for JsValidationResult {
    fn inner(&self) -> &ValidationResult {
        &self.inner
    }
}
