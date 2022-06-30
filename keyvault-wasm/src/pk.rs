use super::*;

/// Multicipher public key
///
/// A public key (also called shared key or pk in some literature) is that part
/// of an asymmetric keypair which can be used to verify the authenticity of the
/// sender of a message or to encrypt a message that can only be decrypted by a
/// single recipient. In both cases this other party owns the {@link PrivateKey}
/// part of the keypair and never shares it with anyone else.
#[wasm_bindgen(js_name = PublicKey)]
#[derive(Clone, Debug)]
pub struct JsMPublicKey {
    inner: MPublicKey,
}

#[wasm_bindgen(js_class = PublicKey)]
impl JsMPublicKey {
    /// Parses a string into a {@link PublicKey}.
    #[wasm_bindgen(constructor)]
    pub fn new(pub_key_str: &str) -> Result<JsMPublicKey, JsValue> {
        let inner: MPublicKey = pub_key_str.parse().map_err_to_js()?;
        Ok(Self { inner })
    }

    /// Converts a {@link SecpPublicKey} into a multicipher {@link PublicKey}.
    #[wasm_bindgen(js_name = fromSecp)]
    pub fn from_secp(pk: &JsSecpPublicKey) -> Self {
        let inner = MPublicKey::from(pk.inner().clone());
        Self { inner }
    }

    /// All multicipher public keys start with this prefix
    #[wasm_bindgen]
    pub fn prefix() -> String {
        MPublicKey::PREFIX.to_string()
    }

    /// Calculates the key id (also called fingerprint or address in some
    /// literature) of the public key.
    #[wasm_bindgen(js_name = keyId)]
    pub fn key_id(&self) -> JsMKeyId {
        JsMKeyId::from(self.inner.key_id())
    }

    /// Validates if `key_id` belongs to this public key
    ///
    /// We do not yet have multiple versions of key ids for the same multicipher
    /// public key, so for now this comparison is trivial. But when we introduce
    /// newer versions, we need to take the version of the `key_id` argument
    /// into account and calculate that possibly older version from `self`.
    #[wasm_bindgen(js_name = validateId)]
    pub fn validate_id(&self, key_id: &JsMKeyId) -> bool {
        self.inner.validate_id(key_id.inner())
    }

    /// This method can be used to verify if a given signature for a message was
    /// made using the private key that belongs to this public key.
    ///
    /// @see PrivateKey.sign
    #[wasm_bindgen(js_name = validateEcdsa)]
    pub fn validate_ecdsa(&self, data: &[u8], signature: &JsMSignature) -> bool {
        self.inner.verify(data, signature.inner())
    }

    /// Converts a {@link PublicKey} into a string.
    // Note that Clippy complains if you call these methods to_string. But implementing Display is not enough to get a toString in JS.
    #[wasm_bindgen(js_name=toString)]
    pub fn stringify(&self) -> String {
        self.inner.to_string()
    }
}

impl From<MPublicKey> for JsMPublicKey {
    fn from(inner: MPublicKey) -> Self {
        Self { inner }
    }
}

impl Wraps<MPublicKey> for JsMPublicKey {
    fn inner(&self) -> &MPublicKey {
        &self.inner
    }
}

/// Secp256k1 public key
#[wasm_bindgen(js_name = SecpPublicKey)]
#[derive(Clone, Debug)]
pub struct JsSecpPublicKey {
    inner: SecpPublicKey,
}

#[wasm_bindgen(js_class = SecpPublicKey)]
impl JsSecpPublicKey {
    /// Parses a string into a {@link SecpPublicKey}.
    #[wasm_bindgen(constructor)]
    pub fn new(key: &str) -> Result<JsSecpPublicKey, JsValue> {
        let inner: SecpPublicKey = key.parse().map_err_to_js()?;
        Ok(Self { inner })
    }

    /// Calculates the key id (also called fingerprint or address in some
    /// literature) of the public key.
    #[wasm_bindgen(js_name = keyId)]
    pub fn key_id(&self) -> JsSecpKeyId {
        JsSecpKeyId::from(self.inner.key_id())
    }

    /// Calculates the key id of the public key the non-standard way ark.io and
    /// therefore Hydra uses.
    ///
    /// Regular bitcoin-based chains use the ripemd160 hash of the sha2-256 hash
    /// of the public key, but ARK only uses ripemd160.
    #[wasm_bindgen(js_name = arkKeyId)]
    pub fn ark_key_id(&self) -> JsSecpKeyId {
        JsSecpKeyId::from(self.inner.ark_key_id())
    }

    /// Validates if `key_id` belongs to this public key
    #[wasm_bindgen(js_name = validateId)]
    pub fn validate_id(&self, key_id: &JsSecpKeyId) -> bool {
        self.inner.validate_id(key_id.inner())
    }

    /// Validates if `key_id` belongs to this public key if it was generated
    /// the ark.io way.
    #[wasm_bindgen(js_name = validateArkId)]
    pub fn validate_ark_id(&self, key_id: &JsSecpKeyId) -> bool {
        self.inner.validate_ark_id(key_id.inner())
    }

    /// This method can be used to verify if a given signature for a message was
    /// made using the private key that belongs to this public key.
    #[wasm_bindgen(js_name = validateEcdsa)]
    pub fn validate_ecdsa(&self, data: &[u8], signature: &JsSecpSignature) -> bool {
        self.inner.verify(data, signature.inner())
    }

    /// Converts a {@link SecpPublicKey} into a string.
    // Note that Clippy complains if you call these methods to_string. But implementing Display is not enough to get a toString in JS.
    #[wasm_bindgen(js_name=toString)]
    pub fn stringify(&self) -> String {
        self.inner.to_string()
    }
}

impl From<SecpPublicKey> for JsSecpPublicKey {
    fn from(inner: SecpPublicKey) -> Self {
        Self { inner }
    }
}

impl Wraps<SecpPublicKey> for JsSecpPublicKey {
    fn inner(&self) -> &SecpPublicKey {
        &self.inner
    }
}
