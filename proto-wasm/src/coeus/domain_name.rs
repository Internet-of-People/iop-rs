use super::*;

#[wasm_bindgen(js_name = DomainName)]
pub struct JsDomainName {
    inner: DomainName,
}

#[wasm_bindgen(js_class = DomainName)]
impl JsDomainName {
    #[wasm_bindgen(constructor)]
    pub fn new(domain_name: &str) -> Result<JsDomainName, JsValue> {
        let inner = domain_name.parse::<DomainName>().map_err_to_js()?;
        Ok(Self { inner })
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn stringify(&self) -> String {
        self.inner.to_string()
    }
}

impl From<DomainName> for JsDomainName {
    fn from(inner: DomainName) -> Self {
        Self { inner }
    }
}

impl Wraps<DomainName> for JsDomainName {
    fn inner(&self) -> &DomainName {
        &self.inner
    }
}
