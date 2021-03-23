use super::*;

#[wasm_bindgen(js_name = Signature)]
#[derive(Clone, Debug)]
pub struct JsMSignature {
    inner: MSignature,
}

#[wasm_bindgen(js_class = Signature)]
impl JsMSignature {
    #[wasm_bindgen(constructor)]
    pub fn new(sign_str: &str) -> Result<JsMSignature, JsValue> {
        let inner: MSignature = sign_str.parse().map_err_to_js()?;
        Ok(Self { inner })
    }

    #[wasm_bindgen(js_name = fromSecp)]
    pub fn from_secp(secp: &JsSecpSignature) -> Self {
        let inner = MSignature::from(secp.inner().clone());
        Self { inner }
    }

    #[wasm_bindgen]
    pub fn prefix() -> String {
        MSignature::PREFIX.to_string()
    }

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

#[wasm_bindgen(js_name = SecpSignature)]
#[derive(Clone, Debug)]
pub struct JsSecpSignature {
    inner: SecpSignature,
}

#[wasm_bindgen(js_class = SecpSignature)]
impl JsSecpSignature {
    #[wasm_bindgen(js_name = fromDer)]
    pub fn from_der(bytes: &[u8]) -> Result<JsSecpSignature, JsValue> {
        let inner = SecpSignature::from_der(bytes).map_err_to_js()?;
        Ok(Self { inner })
    }

    #[wasm_bindgen(js_name = toDer)]
    pub fn to_der(&self) -> Vec<u8> {
        self.inner.to_der()
    }

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
