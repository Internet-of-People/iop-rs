use super::*;

#[wasm_bindgen(js_name = Price)]
pub struct JsPrice {
    inner: Price,
}

#[wasm_bindgen(js_class = Price)]
impl JsPrice {
    #[wasm_bindgen(getter)]
    pub fn fee(&self) -> u64 {
        self.inner.fee
    }
}

impl From<Price> for JsPrice {
    fn from(inner: Price) -> Self {
        Self { inner }
    }
}

impl Wraps<Price> for JsPrice {
    fn inner(&self) -> &Price {
        &self.inner
    }
}
