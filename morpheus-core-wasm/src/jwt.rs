use super::*;

#[wasm_bindgen(js_name = JwtBuilder)]
#[derive(Default)]
pub struct JsJwtBuilder {
    inner: JwtBuilder,
}

#[wasm_bindgen(js_class = JwtBuilder)]
impl JsJwtBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsJwtBuilder {
        let inner = JwtBuilder::default();
        JsJwtBuilder { inner }
    }

    #[wasm_bindgen(js_name = withContentId)]
    pub fn with_content_id(content_id: &str) -> JsJwtBuilder {
        let inner = JwtBuilder::with_content_id(content_id.to_owned());
        JsJwtBuilder { inner }
    }

    #[wasm_bindgen]
    pub fn sign(&self, sk: &JsMPrivateKey) -> Result<String, JsValue> {
        let token = self.inner.sign(sk.inner()).map_err_to_js()?;
        Ok(token)
    }
}

#[wasm_bindgen(js_name = JwtParser)]
pub struct JsJwtParser {
    inner: JwtParser,
}

#[wasm_bindgen(js_class = JwtParser)]
impl JsJwtParser {
    #[wasm_bindgen(constructor)]
    pub fn new(token: &str) -> Result<JsJwtParser, JsValue> {
        let inner = JwtParser::new(token, None).map_err_to_js()?;
        Ok(JsJwtParser { inner })
    }

    #[wasm_bindgen(getter = publicKey)]
    pub fn public_key(&self) -> JsMPublicKey {
        JsMPublicKey::from(self.inner.public_key())
    }

    #[wasm_bindgen(getter = createdAt)]
    pub fn created_at(&self) -> i64 {
        self.inner.created_at().timestamp()
    }

    #[wasm_bindgen(getter = timeToLive)]
    pub fn time_to_live(&self) -> i64 {
        self.inner.time_to_live().num_seconds()
    }
}
