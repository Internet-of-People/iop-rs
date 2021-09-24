use super::*;

#[wasm_bindgen(js_name = CoeusAsset)]
pub struct JsCoeusAsset {
    inner: CoeusAsset,
}

#[wasm_bindgen(js_class = CoeusAsset)]
impl JsCoeusAsset {
    #[wasm_bindgen(constructor)]
    pub fn new(data: &JsValue) -> Result<JsCoeusAsset, JsValue> {
        let inner: CoeusAsset = data.clone().into_serde().map_err_to_js()?;
        Ok(inner.into())
    }

    pub fn deserialize(bytes: &[u8]) -> Result<JsCoeusAsset, JsValue> {
        let inner = CoeusAsset::from_bytes(bytes).map_err_to_js()?;
        Ok(inner.into())
    }

    pub fn serialize(&self) -> Result<Vec<u8>, JsValue> {
        self.inner.to_bytes().map_err_to_js()
    }

    pub fn fee(&self) -> u64 {
        self.inner.fee()
    }

    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.inner).map_err_to_js()
    }
}

impl From<CoeusAsset> for JsCoeusAsset {
    fn from(inner: CoeusAsset) -> Self {
        Self { inner }
    }
}

impl Wraps<CoeusAsset> for JsCoeusAsset {
    fn inner(&self) -> &CoeusAsset {
        &self.inner
    }
}
