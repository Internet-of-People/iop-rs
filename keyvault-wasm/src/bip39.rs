use iop_keyvault::{Bip39, Bip39Phrase};
use wasm_bindgen::prelude::*;

use super::*;

#[wasm_bindgen(js_name = Bip39)]
#[derive(Clone, Debug)]
pub struct JsBip39 {
    inner: Bip39,
}

#[wasm_bindgen(js_class = Bip39)]
impl JsBip39 {
    #[wasm_bindgen(constructor)]
    pub fn new(lang_code: &str) -> Result<JsBip39, JsValue> {
        let inner = Bip39::language_code(lang_code).map_err(err_to_js)?;
        Ok(Self { inner })
    }

    // NOTE Mnemonic::new() generates random entropy which accesses OsRng.
    //      Os access is refused from sandboxes like NodeJs Wasm.
    //      So to be usable, we need entropy as explicit input instead.
    #[wasm_bindgen(js_name = entropy)]
    pub fn entropy(&self, entropy: &[u8]) -> Result<JsBip39Phrase, JsValue> {
        let phrase = self.inner.entropy(entropy).map_err(err_to_js)?;
        Ok(JsBip39Phrase::from(phrase))
    }

    #[wasm_bindgen(js_name = validatePhrase)]
    pub fn validate_phrase(&self, phrase: &str) -> Result<(), JsValue> {
        self.inner.validate(phrase).map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = listWords)]
    pub fn list_words(&self, prefix: &str) -> Box<[JsValue]> {
        let words = self
            .inner
            .list_words(prefix)
            .iter()
            .map(|word| JsValue::from_str(word))
            .collect::<Vec<_>>();
        words.into_boxed_slice()
    }

    pub fn phrase(&self, phrase: &str) -> Result<JsBip39Phrase, JsValue> {
        let phrase = self.inner.phrase(phrase).map_err(err_to_js)?;
        Ok(JsBip39Phrase::from(phrase))
    }
}

#[wasm_bindgen(js_name = Bip39Phrase)]
pub struct JsBip39Phrase {
    inner: Bip39Phrase,
}

#[wasm_bindgen(js_class = Bip39Phrase)]
impl JsBip39Phrase {
    pub fn password(&self, password: &str) -> JsSeed {
        JsSeed::from(self.inner.password(password))
    }

    #[wasm_bindgen(getter = phrase)]
    pub fn phrase(&self) -> String {
        self.inner.as_phrase().to_string()
    }
}

impl From<Bip39Phrase> for JsBip39Phrase {
    fn from(inner: Bip39Phrase) -> Self {
        Self { inner }
    }
}

impl Wraps<Bip39Phrase> for JsBip39Phrase {
    fn inner(&self) -> &Bip39Phrase {
        &self.inner
    }
}
