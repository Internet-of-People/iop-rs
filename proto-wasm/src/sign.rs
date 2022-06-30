use super::*;

/// Binary data signed by a multicipher key.
#[wasm_bindgen(js_name = SignedBytes)]
pub struct JsSignedBytes {
    inner: Signed<Box<[u8]>>,
}

#[wasm_bindgen(js_class = SignedBytes)]
impl JsSignedBytes {
    /// Create {@link SignedBytes} from its parts.
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

    /// Accessor of the {@link PublicKey} that signed the binary data.
    #[wasm_bindgen(getter, js_name = publicKey)]
    pub fn public_key(&self) -> JsMPublicKey {
        JsMPublicKey::from(self.inner.public_key().to_owned())
    }

    /// Accessor of the binary data.
    #[wasm_bindgen(getter)]
    pub fn content(&self) -> Box<[u8]> {
        self.inner.content().clone()
    }

    /// Accessor of the {@link Signature}.
    #[wasm_bindgen(getter)]
    pub fn signature(&self) -> JsMSignature {
        JsMSignature::from(self.inner.signature().to_owned())
    }

    /// Verify if {@link signature} was made by the private key that belongs to {@link publicKey} on the {@link content}.
    #[wasm_bindgen]
    pub fn validate(&self) -> bool {
        self.inner.validate()
    }

    /// Not only validate the signature, but also check if the provided {@link KeyId} was made from {@link publicKey}.
    ///
    /// @see validate
    #[wasm_bindgen(js_name = validateWithKeyId)]
    pub fn validate_with_keyid(&self, signer_id: &JsMKeyId) -> bool {
        self.inner.validate_with_keyid(Some(signer_id.inner()))
    }

    /// Not only validate the signature, but also check the signing key had impersonation right the whole time period specified by the
    /// optional upper and lower block height boundaries. The DID document serialized as a string provides the whole history of key
    /// rights, so depending on the use-case there are three possible outcomes:
    ///
    /// - The signing key had impersonation right the whole time and the signature is valid (green)
    /// - Cannot prove if the signing key had impersonation right the whole time, but no other issues found (yellow)
    /// - The signature is invalid or we can prove the signing key did not have impersonation right at any point in
    ///   the given time interval (red)
    ///
    /// The return value is a {@link ValidationResult}
    #[wasm_bindgen(js_name = validateWithDidDoc)]
    pub fn validate_with_did_doc(
        &self, did_doc_str: &str, from_height_inc: Option<BlockHeight>,
        until_height_exc: Option<BlockHeight>,
    ) -> Result<JsValue, JsValue> {
        validate_with_did_doc(&self.inner, did_doc_str, from_height_inc, until_height_exc)
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

/// JSON signed by a multicipher key. Since the signature is done on a digest created by {@link digestJson}, the same signature can be
/// validated against different selectively revealed JSON documents.
///
/// @see selectiveDigestJson
#[wasm_bindgen(js_name = SignedJson)]
pub struct JsSignedJson {
    inner: Signed<serde_json::Value>,
}

#[wasm_bindgen(js_class = SignedJson)]
impl JsSignedJson {
    /// Create {@link SignedJson} from its parts.
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

    /// Accessor of the {@link PublicKey} that signed the binary data.
    #[wasm_bindgen(getter, js_name = publicKey)]
    pub fn public_key(&self) -> JsMPublicKey {
        JsMPublicKey::from(self.inner.public_key().to_owned())
    }

    /// Accessor of the JSON content.
    #[wasm_bindgen(getter)]
    pub fn content(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(self.inner.content()).map_err_to_js()
    }

    /// Accessor of the {@link Signature}.
    #[wasm_bindgen(getter)]
    pub fn signature(&self) -> JsMSignature {
        JsMSignature::from(self.inner.signature().to_owned())
    }

    /// Verify if {@link signature} was made by the private key that belongs to {@link publicKey} on the {@link content}.
    #[wasm_bindgen]
    pub fn validate(&self) -> bool {
        self.inner.validate()
    }

    /// Not only validate the signature, but also check if the provided {@link KeyId} was made from {@link publicKey}.
    ///
    /// @see validate
    #[wasm_bindgen(js_name = validateWithKeyId)]
    pub fn validate_with_keyid(&self, signer_id: &JsMKeyId) -> bool {
        self.inner.validate_with_keyid(Some(signer_id.inner()))
    }

    /// Not only validate the signature, but also check the signing key had impersonation right the whole time period specified by the
    /// optional upper and lower block height boundaries. The DID document serialized as a string provides the whole history of key
    /// rights, so depending on the use-case there are three possible outcomes:
    ///
    /// - The signing key had impersonation right the whole time and the signature is valid (green)
    /// - Cannot prove if the signing key had impersonation right the whole time, but no other issues found (yellow)
    /// - The signature is invalid or we can prove the signing key did not have impersonation right at any point in
    ///   the given time interval (red)
    ///
    /// The return value is a {@link ValidationResult}
    #[wasm_bindgen(js_name = validateWithDidDoc)]
    pub fn validate_with_did_doc(
        &self, did_doc_str: &str, from_height_inc: Option<BlockHeight>,
        until_height_exc: Option<BlockHeight>,
    ) -> Result<JsValue, JsValue> {
        validate_with_did_doc(&self.inner, did_doc_str, from_height_inc, until_height_exc)
    }

    /// Serialize this object as a JSON in a format used by IOP SSI in several places
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.inner).map_err_to_js()
    }

    /// Deserialize a {@SignedJson} from a JSON in a format used by IOP SSI in several places
    #[wasm_bindgen(js_name = fromJSON)]
    pub fn from_json(json: &JsValue) -> Result<JsSignedJson, JsValue> {
        let parsed: Signed<serde_json::Value> = json.into_serde().map_err_to_js()?;
        Ok(parsed.into())
    }
}

impl From<Signed<serde_json::Value>> for JsSignedJson {
    fn from(inner: Signed<serde_json::Value>) -> Self {
        Self { inner }
    }
}

impl Wraps<Signed<serde_json::Value>> for JsSignedJson {
    fn inner(&self) -> &Signed<serde_json::Value> {
        &self.inner
    }
}

fn validate_with_did_doc<T: Signable>(
    signed: &Signed<T>, did_doc_str: &str, from_height_inc: Option<BlockHeight>,
    until_height_exc: Option<BlockHeight>,
) -> Result<JsValue, JsValue> {
    let did_doc = serde_json::from_str(did_doc_str).map_err_to_js()?;
    let result = signed
        .validate_with_did_doc(&did_doc, from_height_inc, until_height_exc)
        .map_err_to_js()?;
    Ok(JsValidationResult { inner: result }.into())
}

/// A single issue found while validating against a DID document.
///
/// @see SignedBytes.validateWithDidDoc, SignedJson.validateWithDidDoc
#[wasm_bindgen(js_name = ValidationIssue)]
pub struct JsValidationIssue {
    inner: ValidationIssue,
}

#[wasm_bindgen(js_class = ValidationIssue)]
impl JsValidationIssue {
    /// Error code of the issue
    #[wasm_bindgen(getter)]
    pub fn code(&self) -> u32 {
        self.inner.code()
    }

    /// Severity of the issue ('warning' or 'error')
    #[wasm_bindgen(getter)]
    pub fn severity(&self) -> String {
        self.inner.severity().to_string()
    }

    /// Description of the issue as a string
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

/// All issues found while validating against a DID document.
///
/// @see SignedBytes.validateWithDidDoc, SignedJson.validateWithDidDoc
#[wasm_bindgen(js_name = ValidationResult)]
pub struct JsValidationResult {
    inner: ValidationResult,
}

#[wasm_bindgen(js_class = ValidationResult)]
impl JsValidationResult {
    /// Status of the validation based on the highest severity found among the issues ('invalid', 'maybe valid' or 'valid')
    #[wasm_bindgen(getter)]
    pub fn status(&self) -> String {
        self.inner.status().to_string()
    }

    /// An array of all issues. Treat each item as a {@link ValidationIssue}.
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
