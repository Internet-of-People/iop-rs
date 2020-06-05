use iop_keyvault::Seed;
use wasm_bindgen::prelude::*;

use super::*;

#[wasm_bindgen(js_name = Seed)]
pub struct JsSeed {
    inner: Seed,
}

#[wasm_bindgen(js_class = Seed)]
impl JsSeed {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: &[u8]) -> Result<JsSeed, JsValue> {
        let inner = Seed::from_bytes(bytes).map_err(err_to_js)?;
        Ok(Self { inner })
    }

    #[wasm_bindgen(js_name = demoPhrase)]
    pub fn demo_phrase() -> String {
        Seed::DEMO_PHRASE.to_owned()
    }

    #[wasm_bindgen(js_name = legacyPassword)]
    pub fn legacy_password() -> String {
        Seed::PASSWORD.to_owned()
    }

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
