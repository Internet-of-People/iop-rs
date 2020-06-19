use super::*;

#[wasm_bindgen(js_name = KeyId)]
#[derive(Clone, Debug)]
pub struct JsMKeyId {
    inner: MKeyId,
}

#[wasm_bindgen(js_class = KeyId)]
impl JsMKeyId {
    #[wasm_bindgen(constructor)]
    pub fn new(key_id_str: &str) -> Result<JsMKeyId, JsValue> {
        let inner: MKeyId = key_id_str.parse().map_err_to_js()?;
        Ok(Self { inner })
    }

    #[wasm_bindgen(js_name = fromSecp)]
    pub fn from_secp(secp: &JsSecpKeyId) -> Self {
        let inner = MKeyId::from(secp.inner().clone());
        Self { inner }
    }

    #[wasm_bindgen]
    pub fn prefix() -> String {
        MKeyId::PREFIX.to_string()
    }

    // Note that Clippy complains if you call these methods to_string. But implementing Display is not enough to get a toString in JS.
    #[wasm_bindgen(js_name=toString)]
    pub fn stringify(&self) -> String {
        self.inner.to_string()
    }
}

impl From<MKeyId> for JsMKeyId {
    fn from(inner: MKeyId) -> Self {
        Self { inner }
    }
}

impl Wraps<MKeyId> for JsMKeyId {
    fn inner(&self) -> &MKeyId {
        &self.inner
    }
}

#[wasm_bindgen(js_name = SecpKeyId)]
#[derive(Clone, Debug)]
pub struct JsSecpKeyId {
    inner: SecpKeyId,
}

impl From<SecpKeyId> for JsSecpKeyId {
    fn from(inner: SecpKeyId) -> Self {
        Self { inner }
    }
}

impl Wraps<SecpKeyId> for JsSecpKeyId {
    fn inner(&self) -> &SecpKeyId {
        &self.inner
    }
}
