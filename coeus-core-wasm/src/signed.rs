use super::*;

#[wasm_bindgen(js_name = NoncedOperationsBuilder)]
pub struct JsNoncedOperationsBuilder {
    operations: Vec<UserOperation>,
}

#[wasm_bindgen(js_class = NoncedOperationsBuilder)]
impl JsNoncedOperationsBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsNoncedOperationsBuilder {
        Self { operations: Default::default() }
    }

    pub fn add(mut self, operation: &JsOperation) -> Result<JsNoncedOperationsBuilder, JsValue> {
        if let Operation::User(user_op) = operation.inner() {
            self.operations.push(user_op.to_owned());
        } else {
            return Err("NoncedOperations may contain only user operations").map_err_to_js();
        }
        Ok(self)
    }

    pub fn build(self, nonce: Nonce) -> JsNoncedOperations {
        NoncedOperations::new(self.operations, nonce).into()
    }
}

#[wasm_bindgen(js_name = NoncedOperations)]
pub struct JsNoncedOperations {
    inner: NoncedOperations,
}

#[wasm_bindgen(js_class = NoncedOperations)]
impl JsNoncedOperations {
    pub fn price(&self, state: &JsState) -> JsPrice {
        let _state = state.inner();
        self.inner.get_price().into()
    }

    pub fn sign(self, sk: &JsMPrivateKey) -> Result<JsSignedOperations, JsValue> {
        let signed = self.inner.sign(sk.inner()).map_err_to_js()?;
        Ok(signed.into())
    }

    pub fn serialize(&self) -> Result<String, JsValue> {
        self.inner().serialize().map_err_to_js()
    }
}

impl From<NoncedOperations> for JsNoncedOperations {
    fn from(inner: NoncedOperations) -> Self {
        Self { inner }
    }
}

impl Wraps<NoncedOperations> for JsNoncedOperations {
    fn inner(&self) -> &NoncedOperations {
        &self.inner
    }
}

#[wasm_bindgen(js_name = SignedOperations)]
pub struct JsSignedOperations {
    inner: SignedOperations,
}

#[wasm_bindgen(js_class = SignedOperations)]
impl JsSignedOperations {
    pub fn price(&self, state: &JsState) -> JsPrice {
        let _state = state.inner();
        self.inner.get_price().into()
    }
    pub fn verify(&self) -> bool {
        self.inner.verify()
    }
}

impl From<SignedOperations> for JsSignedOperations {
    fn from(inner: SignedOperations) -> Self {
        Self { inner }
    }
}

impl Wraps<SignedOperations> for JsSignedOperations {
    fn inner(&self) -> &SignedOperations {
        &self.inner
    }
}
