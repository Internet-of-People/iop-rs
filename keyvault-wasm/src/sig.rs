use super::*;

/// Multicipher signature
#[wasm_bindgen(js_name = Signature)]
#[derive(Clone, Debug)]
pub struct JsMSignature {
    inner: MSignature,
}

#[wasm_bindgen(js_class = Signature)]
impl JsMSignature {
    /// Parses a string into a {@link Signature}.
    #[wasm_bindgen(constructor)]
    pub fn new(sign_str: &str) -> Result<JsMSignature, JsValue> {
        let inner: MSignature = sign_str.parse().map_err_to_js()?;
        Ok(Self { inner })
    }

    /// Converts a {@link SecpSignature} into a multicipher {@link Signature}.
    #[wasm_bindgen(js_name = fromSecp)]
    pub fn from_secp(secp: &JsSecpSignature) -> Self {
        let inner = MSignature::from(secp.inner().clone());
        Self { inner }
    }

    /// All multicipher signatures start with this prefix
    #[wasm_bindgen]
    pub fn prefix() -> String {
        MSignature::PREFIX.to_string()
    }

    /// Converts a {@link Signature} into a string.
    // Note that Clippy complains if you call these methods to_string. But implementing Display is not enough to get a toString in JS.
    #[wasm_bindgen(js_name=toString)]
    pub fn stringify(&self) -> String {
        self.inner.to_string()
    }
}

impl From<MSignature> for JsMSignature {
    fn from(inner: MSignature) -> Self {
        Self { inner }
    }
}

impl Wraps<MSignature> for JsMSignature {
    fn inner(&self) -> &MSignature {
        &self.inner
    }
}

/// Secp256k1 signature
#[wasm_bindgen(js_name = SecpSignature)]
#[derive(Clone, Debug)]
pub struct JsSecpSignature {
    inner: SecpSignature,
}

#[wasm_bindgen(js_class = SecpSignature)]
impl JsSecpSignature {
    /// Deserializes an ASN.1 DER encoded buffer into a {@link SepcSignature}
    #[wasm_bindgen(js_name = fromDer)]
    pub fn from_der(bytes: &[u8]) -> Result<JsSecpSignature, JsValue> {
        let inner = SecpSignature::from_der(bytes).map_err_to_js()?;
        Ok(Self { inner })
    }

    /// Serializes a {@link SepcSignature} into an ASN.1 DER encoded buffer
    #[wasm_bindgen(js_name = toDer)]
    pub fn to_der(&self) -> Vec<u8> {
        self.inner.to_der()
    }

    /// Converts a {@link SecpSignature} into a string.
    // Note that Clippy complains if you call these methods to_string. But implementing Display is not enough to get a toString in JS.
    #[wasm_bindgen(js_name=toString)]
    pub fn stringify(&self) -> String {
        self.inner.to_string()
    }
}

impl From<SecpSignature> for JsSecpSignature {
    fn from(inner: SecpSignature) -> Self {
        Self { inner }
    }
}

impl Wraps<SecpSignature> for JsSecpSignature {
    fn inner(&self) -> &SecpSignature {
        &self.inner
    }
}
