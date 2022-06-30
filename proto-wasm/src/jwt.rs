use super::*;
use chrono::Duration;

/// Builder object for creating and signing a JWT (JSON Web Token) with or without an associated content.
///
/// @see JwtParser
#[wasm_bindgen(js_name = JwtBuilder)]
#[derive(Default)]
pub struct JsJwtBuilder {
    inner: JwtBuilder,
}

#[wasm_bindgen(js_class = JwtBuilder)]
impl JsJwtBuilder {
    /// Creates a new JWT without an associated content. The time of creation is set
    /// in this call.
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsJwtBuilder {
        let inner = JwtBuilder::default();
        JsJwtBuilder { inner }
    }

    /// Creates a new JWT without an associated content. The time of creation is set
    /// in this call.
    #[wasm_bindgen(js_name = withContentId)]
    pub fn with_content_id(content_id: &str) -> JsJwtBuilder {
        let inner = JwtBuilder::with_content_id(content_id.to_owned());
        JsJwtBuilder { inner }
    }

    /// Gets how long the token is valid. (5 seconds by default)
    #[wasm_bindgen(getter, js_name = timeToLive)]
    pub fn get_time_to_live(&self) -> i64 {
        self.inner.time_to_live.num_seconds()
    }

    /// Sets how long the token is valid.
    #[wasm_bindgen(setter, js_name = timeToLive)]
    pub fn set_time_to_live(&mut self, seconds: i64) {
        self.inner.time_to_live = Duration::seconds(seconds);
    }

    /// Signs and serializes the token with the given multicipher {@link PrivateKey}
    #[wasm_bindgen]
    pub fn sign(&self, sk: &JsMPrivateKey) -> Result<String, JsValue> {
        let token = self.inner.sign(sk.inner()).map_err_to_js()?;
        Ok(token)
    }
}

/// Parser for reading a JWT (JSON Web Token) from a string and validate its content and signature.
#[wasm_bindgen(js_name = JwtParser)]
pub struct JsJwtParser {
    inner: JwtParser,
}

#[wasm_bindgen(js_class = JwtParser)]
impl JsJwtParser {
    /// Parse JWT from a string created with {@link JwtBuilder}
    #[wasm_bindgen(constructor)]
    pub fn new(token: &str) -> Result<JsJwtParser, JsValue> {
        let inner = JwtParser::new(token, None).map_err_to_js()?;
        Ok(JsJwtParser { inner })
    }

    /// Returns the public key that signed the token
    #[wasm_bindgen(getter = publicKey)]
    pub fn public_key(&self) -> JsMPublicKey {
        JsMPublicKey::from(self.inner.public_key())
    }

    /// Returns the UTC date-time instance the token was created
    #[wasm_bindgen(getter = createdAt)]
    pub fn created_at(&self) -> i64 {
        self.inner.created_at().timestamp()
    }

    /// Returns how long the token stays valid
    #[wasm_bindgen(getter = timeToLive)]
    pub fn time_to_live(&self) -> i64 {
        self.inner.time_to_live().num_seconds()
    }
}
