use super::*;

#[wasm_bindgen(js_name = Principal)]
pub struct JsPrincipal {
    inner: Principal,
}

#[wasm_bindgen(js_class = Principal)]
impl JsPrincipal {
    pub fn system() -> JsPrincipal {
        Principal::system().into()
    }

    #[wasm_bindgen(js_name = publicKey)]
    pub fn public_key(pk: &JsMPublicKey) -> Result<JsPrincipal, JsValue> {
        let principal = Principal::public_key(pk.inner());
        Ok(principal.into())
    }

    #[wasm_bindgen(js_name = validateImpersonation)]
    pub fn validate_impersonation(&self, pk: &JsMPublicKey) -> Result<(), JsValue> {
        self.inner.validate_impersonation(pk.inner()).map_err_to_js()
    }
}

impl From<Principal> for JsPrincipal {
    fn from(inner: Principal) -> Self {
        Self { inner }
    }
}

impl Wraps<Principal> for JsPrincipal {
    fn inner(&self) -> &Principal {
        &self.inner
    }
}
