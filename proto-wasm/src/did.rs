use super::*;

#[wasm_bindgen(js_name = Did)]
pub struct JsDid {
    inner: Did,
}

#[wasm_bindgen(js_class = Did)]
impl JsDid {
    #[wasm_bindgen(constructor)]
    pub fn new(did_str: &str) -> Result<JsDid, JsValue> {
        let inner: Did = did_str.parse().map_err_to_js()?;
        Ok(Self { inner })
    }

    #[wasm_bindgen]
    pub fn prefix() -> String {
        Did::PREFIX.to_owned()
    }

    #[wasm_bindgen(js_name = fromKeyId)]
    pub fn from_key_id(key_id: &JsMKeyId) -> Self {
        Did::from(key_id.inner()).into()
    }

    #[wasm_bindgen(js_name = defaultKeyId)]
    pub fn default_key_id(&self) -> JsMKeyId {
        JsMKeyId::from(self.inner.default_key_id())
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn stringify(&self) -> String {
        self.inner.to_string()
    }
}

impl From<Did> for JsDid {
    fn from(inner: Did) -> Self {
        Self { inner }
    }
}

impl Wraps<Did> for JsDid {
    fn inner(&self) -> &Did {
        &self.inner
    }
}
