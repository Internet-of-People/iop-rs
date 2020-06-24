use super::*;

#[wasm_bindgen(js_name = HydraParameters)]
pub struct JsHydraParameters {
    inner: HydraParameters,
}

#[wasm_bindgen(js_class = HydraParameters)]
impl JsHydraParameters {
    #[wasm_bindgen(constructor)]
    pub fn new(network: &str, account: i32) -> Result<JsHydraParameters, JsValue> {
        let network = Networks::by_name(network).map_err_to_js()?;
        let inner = HydraParameters::new(network, account);
        Ok(JsHydraParameters::from(inner))
    }
}

impl From<HydraParameters> for JsHydraParameters {
    fn from(inner: HydraParameters) -> Self {
        Self { inner }
    }
}

impl Wraps<HydraParameters> for JsHydraParameters {
    fn inner(&self) -> &HydraParameters {
        &self.inner
    }
}
