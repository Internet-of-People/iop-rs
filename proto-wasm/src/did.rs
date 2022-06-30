use super::*;

/// An object representing a valid DID. This identifier can be used to look up a DID document
/// on multiple blockchains. Without any on-chain SSI transactions, there will be a single
/// key that can update and impersonate the DID, which has the default key identifier.
///
/// @see defaultKeyId
#[wasm_bindgen(js_name = Did)]
pub struct JsDid {
    inner: Did,
}

#[wasm_bindgen(js_class = Did)]
impl JsDid {
    /// Try to parse a string into a DID
    #[wasm_bindgen(constructor)]
    pub fn new(did_str: &str) -> Result<JsDid, JsValue> {
        let inner: Did = did_str.parse().map_err_to_js()?;
        Ok(Self { inner })
    }

    /// All DID strings start with this prefix
    #[wasm_bindgen]
    pub fn prefix() -> String {
        Did::PREFIX.to_owned()
    }

    /// Creates a DID from a multicipher {@KeyId}, so the default key identifier of the DID
    /// will match that.
    ///
    /// @see defaultKeyId
    #[wasm_bindgen(js_name = fromKeyId)]
    pub fn from_key_id(key_id: &JsMKeyId) -> Self {
        Did::from(key_id.inner()).into()
    }

    /// Returns the default key identifier for a DID that has update and impersonation rights
    /// unless the DID document was modified on chain.
    #[wasm_bindgen(js_name = defaultKeyId)]
    pub fn default_key_id(&self) -> JsMKeyId {
        JsMKeyId::from(self.inner.default_key_id())
    }

    /// Converts the DID into a string like `did:morpheus:ezBlah`
    #[wasm_bindgen(js_name = toString)]
    pub fn stringify(&self) -> String {
        self.inner.to_string()
    }
}

impl From<Did> for JsDid {
    fn from(inner: Did) -> Self {
        Self { inner }
    }
}

impl Wraps<Did> for JsDid {
    fn inner(&self) -> &Did {
        &self.inner
    }
}
