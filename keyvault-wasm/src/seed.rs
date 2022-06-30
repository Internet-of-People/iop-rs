use super::*;

/// The seed used for BIP32 derivations. A seed cannot be turned back into a
/// phrase, because there is salted hashing involed in creating it from the
/// BIP39 mnemonic phrase.
#[wasm_bindgen(js_name = Seed)]
pub struct JsSeed {
    inner: Seed,
}

#[wasm_bindgen(js_class = Seed)]
impl JsSeed {
    /// Creates seed from a raw 512-bit binary seed
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: &[u8]) -> Result<JsSeed, JsValue> {
        let inner = Seed::from_bytes(bytes).map_err_to_js()?;
        Ok(Self { inner })
    }

    /// A BIP39 phrase we use in most of the demo videos and proof-of-concept
    /// applications. Do not use it in production code.
    #[wasm_bindgen(js_name = demoPhrase)]
    pub fn demo_phrase() -> String {
        Seed::DEMO_PHRASE.to_owned()
    }

    /// Legacy password used in the 0.0.1 version of the crate. Since 0.0.2 the
    /// crate always requires a password, which should be "" by default when
    /// the user does not provide one. (BIP39 standard for "25th word")
    #[wasm_bindgen(js_name = legacyPassword)]
    pub fn legacy_password() -> String {
        Seed::PASSWORD.to_owned()
    }

    /// Returns the 512-bit binary representation of the seed
    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.inner.as_bytes().to_vec()
    }
}

impl From<Seed> for JsSeed {
    fn from(inner: Seed) -> Self {
        Self { inner }
    }
}

impl Wraps<Seed> for JsSeed {
    fn inner(&self) -> &Seed {
        &self.inner
    }
}
