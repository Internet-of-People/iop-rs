use super::*;

#[wasm_bindgen(js_name = State)]
pub struct JsState {
    inner: State,
}

#[wasm_bindgen(js_class = State)]
impl JsState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<JsState, JsValue> {
        let inner = State::new();
        Ok(Self { inner })
    }

    #[wasm_bindgen(js_name = resolveData)]
    pub fn resolve_data(&self, name: &JsDomainName) -> Result<JsValue, JsValue> {
        let data = self.inner.resolve_data(name.inner()).map_err_to_js()?;
        JsValue::from_serde(data).map_err_to_js()
    }

    #[wasm_bindgen(js_name = applySignedOperations)]
    pub fn apply_signed_operations(
        &mut self, ops: &JsSignedOperations,
    ) -> Result<Version, JsValue> {
        self.inner.apply_signed_operations(ops.inner().to_owned()).map_err_to_js()
    }

    #[wasm_bindgen(getter)]
    pub fn version(&self) -> Version {
        self.inner.version()
    }

    #[wasm_bindgen(js_name = undoLastOperation)]
    pub fn undo_last_operation(&mut self, to_version: Version) -> Result<(), JsValue> {
        self.inner.undo_last_operation(to_version).map_err_to_js()
    }

    #[wasm_bindgen(getter = lastSeenHeight)]
    pub fn last_seen_height(&self) -> BlockHeight {
        self.inner.last_seen_height()
    }

    // #[wasm_bindgen(js_name = toString)]
    // pub fn stringify(&self) -> String {
    //     self.inner.to_string()
    // }
}

impl From<State> for JsState {
    fn from(inner: State) -> Self {
        Self { inner }
    }
}

impl Wraps<State> for JsState {
    fn inner(&self) -> &State {
        &self.inner
    }
}
