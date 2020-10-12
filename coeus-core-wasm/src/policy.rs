use super::*;

#[wasm_bindgen(js_name = SubtreePolicies)]
pub struct JsSubtreePolicies {
    inner: SubtreePolicies,
}

#[wasm_bindgen(js_class = SubtreePolicies)]
impl JsSubtreePolicies {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsSubtreePolicies {
        SubtreePolicies::new().into()
    }

    #[wasm_bindgen(js_name = withSchema)]
    pub fn with_schema(self, schema: &JsValue) -> Result<JsSubtreePolicies, JsValue> {
        Ok(self.inner.with_schema(schema.into_serde().map_err_to_js()?).into())
    }

    #[wasm_bindgen(js_name = withExpiration)]
    pub fn with_expiration(self, max_block_count: BlockCount) -> JsSubtreePolicies {
        self.inner.with_expiration(max_block_count).into()
    }
}

impl From<SubtreePolicies> for JsSubtreePolicies {
    fn from(inner: SubtreePolicies) -> Self {
        Self { inner }
    }
}

impl Wraps<SubtreePolicies> for JsSubtreePolicies {
    fn inner(&self) -> &SubtreePolicies {
        &self.inner
    }
}
