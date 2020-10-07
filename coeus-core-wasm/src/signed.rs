use super::*;

#[wasm_bindgen(js_name = NoncedOperations)]
pub struct JsNoncedOperations {
    inner: NoncedOperations,
}

#[wasm_bindgen(js_class = NoncedOperations)]
impl JsNoncedOperations {
    // TODO should receive array of operations or allow appending operations
    #[wasm_bindgen(constructor)]
    pub fn new(operations: &JsOperation, nonce: Nonce) -> Result<JsNoncedOperations, JsValue> {
        if let Operation::User(user_op) = operations.inner() {
            Ok(NoncedOperations::new(vec![user_op.to_owned()], nonce).into())
        } else {
            Err("NoncedOperations may contain only user operations").map_err_to_js()
        }
    }

    pub fn price(&self, state: &JsState) -> JsPrice {
        self.inner.get_price(state.inner()).into()
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
        self.inner.get_price(state.inner()).into()
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
