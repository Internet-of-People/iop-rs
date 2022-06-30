use super::*;

/// Tool for generating, validating and parsing [BIP-0039](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) phrases in different supported languages.
#[wasm_bindgen(js_name = Bip39)]
#[derive(Clone, Debug)]
pub struct JsBip39 {
    inner: Bip39,
}

#[wasm_bindgen(js_class = Bip39)]
impl JsBip39 {
    /// Creates an object that can handle BIP39 phrases in a given language. (e.g. 'en')
    #[wasm_bindgen(constructor)]
    pub fn new(lang_code: &str) -> Result<JsBip39, JsValue> {
        let inner = Bip39::language_code(lang_code).map_err_to_js()?;
        Ok(Self { inner })
    }

    /// Creates a new phrase using the [CSPRNG](https://en.wikipedia.org/wiki/Cryptographically_secure_pseudorandom_number_generator)
    /// available on the platform.
    pub fn generate(&self) -> Result<JsBip39Phrase, JsValue> {
        let phrase = self.inner.generate();
        Ok(JsBip39Phrase::from(phrase))
    }

    /// Creates a new phrase using the 256 bits of entropy provided in a buffer. IOP encourages using 24 word phrases everywhere.
    #[wasm_bindgen(js_name = entropy)]
    pub fn entropy(&self, entropy: &[u8]) -> Result<JsBip39Phrase, JsValue> {
        let phrase = self.inner.entropy(entropy).map_err_to_js()?;
        Ok(JsBip39Phrase::from(phrase))
    }

    /// Creates a new phrase using the entropy provided in a buffer. This method is only for compatibility with other wallets. Check
    /// the BIP39 standard for the buffer sizes allowed.
    #[wasm_bindgen(js_name = shortEntropy)]
    pub fn short_entropy(&self, entropy: &[u8]) -> Result<JsBip39Phrase, JsValue> {
        let phrase = self.inner.short_entropy(entropy).map_err_to_js()?;
        Ok(JsBip39Phrase::from(phrase))
    }

    /// Validates a whole BIP39 mnemonic phrase. Because the phrase contains some checksum, the whole phrase can be invalid even when
    /// each word itself is valid. Note also, that the standards only allows NFKD normalization of Unicode codepoints, and a single
    /// space between words, but this library is more tolerant and provides normalization for those.
    #[wasm_bindgen(js_name = validatePhrase)]
    pub fn validate_phrase(&self, phrase: &str) -> Result<(), JsValue> {
        self.inner.validate(phrase).map_err_to_js()
    }

    /// Lists all words in the BIP39 dictionary, which start with the given prefix.
    ///
    /// Can be used in 3 different ways:
    /// - When the prefix is empty, the sorted list of all words are returned
    /// - When the prefix is a partial word, the returned list can be used for auto-completion
    /// - When the returned list is empty, the prefix is not a valid word in the dictionary
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

    /// Validates a whole 24-word BIP39 mnemonic phrase and returns an intermediate object that can be
    /// later converted into a [`Seed`] with an optional password.
    pub fn phrase(&self, phrase: &str) -> Result<JsBip39Phrase, JsValue> {
        let phrase = self.inner.phrase(phrase).map_err_to_js()?;
        Ok(JsBip39Phrase::from(phrase))
    }

    #[wasm_bindgen(js_name = shortPhrase)]
    /// Validates a whole BIP39 mnemonic phrase and returns an intermediate object similar to {@link phrase}. This method is only for
    /// compatibility with other wallets. Check the BIP39 standard for the number of words allowed.
    pub fn short_phrase(&self, phrase: &str) -> Result<JsBip39Phrase, JsValue> {
        let phrase = self.inner.short_phrase(phrase).map_err_to_js()?;
        Ok(JsBip39Phrase::from(phrase))
    }
}

/// An intermediate object that represents a BIP39 phrase with a known language
#[wasm_bindgen(js_name = Bip39Phrase)]
pub struct JsBip39Phrase {
    inner: Bip39Phrase,
}

#[wasm_bindgen(js_class = Bip39Phrase)]
impl JsBip39Phrase {
    /// Creates a {@link Seed} from the phrase with the given password. Give empty string when the user did not provide any password.
    pub fn password(&self, password: &str) -> JsSeed {
        JsSeed::from(self.inner.password(password))
    }

    /// Returns the phrase as a readable string
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
