use super::*;

/// Multicipher key id (fingerprint/digest/hash of a public key)
///
/// In some algorithms the public key is only revealed in point-to-point
/// communications and a keypair is identified only by the digest of the public
/// key in all other channels.
#[wasm_bindgen(js_name = KeyId)]
#[derive(Clone, Debug)]
pub struct JsMKeyId {
    inner: MKeyId,
}

#[wasm_bindgen(js_class = KeyId)]
impl JsMKeyId {
    /// Parses a string into a {@link KeyId}.
    #[wasm_bindgen(constructor)]
    pub fn new(key_id_str: &str) -> Result<JsMKeyId, JsValue> {
        let inner: MKeyId = key_id_str.parse().map_err_to_js()?;
        Ok(Self { inner })
    }

    /// Converts a {@link SecpKeyId} into a multicipher {@link KeyId}.
    #[wasm_bindgen(js_name = fromSecp)]
    pub fn from_secp(secp: &JsSecpKeyId) -> Self {
        let inner = MKeyId::from(secp.inner().clone());
        Self { inner }
    }

    /// All multicipher key ids start with this prefix
    #[wasm_bindgen]
    pub fn prefix() -> String {
        MKeyId::PREFIX.to_string()
    }

    /// Converts a {@link KeyId} into a string.
    // Note that Clippy complains if you call these methods to_string. But implementing Display is not enough to get a toString in JS.
    #[wasm_bindgen(js_name=toString)]
    pub fn stringify(&self) -> String {
        self.inner.to_string()
    }
}

impl From<MKeyId> for JsMKeyId {
    fn from(inner: MKeyId) -> Self {
        Self { inner }
    }
}

impl Wraps<MKeyId> for JsMKeyId {
    fn inner(&self) -> &MKeyId {
        &self.inner
    }
}

/// Secp256k1 key id (fingerprint/digest/hash of a public key)
#[wasm_bindgen(js_name = SecpKeyId)]
#[derive(Clone, Debug)]
pub struct JsSecpKeyId {
    inner: SecpKeyId,
}

#[wasm_bindgen(js_class = SecpKeyId)]
impl JsSecpKeyId {
    /// Deserializes the key identifier from a `p2pkh` bitcoin address
    #[wasm_bindgen(js_name=fromAddress)]
    pub fn from_p2pkh_addr(address: &str, network: &str) -> Result<JsSecpKeyId, JsValue> {
        let network = Networks::by_name(network).map_err_to_js()?;
        let inner = SecpKeyId::from_p2pkh_addr(address, network).map_err_to_js()?;
        Ok(inner.into())
    }

    /// Serializes the key identifier as a `p2pkh` bitcoin address
    #[wasm_bindgen(js_name=toAddress)]
    pub fn to_p2pkh_addr(&self, network: &str) -> Result<String, JsValue> {
        let network = Networks::by_name(network).map_err_to_js()?;
        Ok(self.inner.to_p2pkh_addr(network.p2pkh_addr()))
    }
}

impl From<SecpKeyId> for JsSecpKeyId {
    fn from(inner: SecpKeyId) -> Self {
        Self { inner }
    }
}

impl Wraps<SecpKeyId> for JsSecpKeyId {
    fn inner(&self) -> &SecpKeyId {
        &self.inner
    }
}
