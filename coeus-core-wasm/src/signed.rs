use super::*;

#[wasm_bindgen(js_name = NoncedBundleBuilder)]
pub struct JsNoncedBundleBuilder {
    operations: Vec<UserOperation>,
}

#[wasm_bindgen(js_class = NoncedBundleBuilder)]
impl JsNoncedBundleBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsNoncedBundleBuilder {
        Self { operations: Default::default() }
    }

    // TODO Alternative design would be to just have a build() that takes a JS array of
    //      JsOperations, and we make sure each item is of correct type and convert it to a Rust Vec
    pub fn add(mut self, user_operation: &JsUserOperation) -> JsNoncedBundleBuilder {
        self.operations.push(user_operation.inner().to_owned());
        self
    }

    pub fn build(self, nonce: Nonce) -> JsNoncedBundle {
        NoncedBundle::new(self.operations, nonce).into()
    }
}

#[wasm_bindgen(js_name = NoncedBundle)]
pub struct JsNoncedBundle {
    inner: NoncedBundle,
}

#[wasm_bindgen(js_class = NoncedBundle)]
impl JsNoncedBundle {
    pub fn price(&self, state: &JsState) -> JsPrice {
        let _state = state.inner();
        self.inner.get_price().into()
    }

    pub fn sign(self, sk: &JsMPrivateKey) -> Result<JsSignedBundle, JsValue> {
        let signed = self.inner.sign(sk.inner()).map_err_to_js()?;
        Ok(signed.into())
    }

    pub fn serialize(&self) -> Result<String, JsValue> {
        self.inner().serialize().map_err_to_js()
    }
}

impl From<NoncedBundle> for JsNoncedBundle {
    fn from(inner: NoncedBundle) -> Self {
        Self { inner }
    }
}

impl Wraps<NoncedBundle> for JsNoncedBundle {
    fn inner(&self) -> &NoncedBundle {
        &self.inner
    }
}

#[wasm_bindgen(js_name = SignedBundle)]
pub struct JsSignedBundle {
    inner: SignedBundle,
}

#[wasm_bindgen(js_class = SignedBundle)]
impl JsSignedBundle {
    #[wasm_bindgen(constructor)]
    pub fn new(data: &JsValue) -> Result<JsSignedBundle, JsValue> {
        let signed_ops: SignedBundle = data.into_serde().map_err_to_js()?;
        Ok(signed_ops.into())
    }

    pub fn price(&self, state: &JsState) -> JsPrice {
        let _state = state.inner();
        self.inner.get_price().into()
    }
    pub fn verify(&self) -> bool {
        self.inner.verify()
    }
}

impl From<SignedBundle> for JsSignedBundle {
    fn from(inner: SignedBundle) -> Self {
        Self { inner }
    }
}

impl Wraps<SignedBundle> for JsSignedBundle {
    fn inner(&self) -> &SignedBundle {
        &self.inner
    }
}
