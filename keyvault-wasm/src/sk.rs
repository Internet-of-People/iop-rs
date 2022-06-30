use super::*;

/// Multicipher private key
///
/// A private key (also called secret key or sk in some literature) is the part of an asymmetric keypair
/// which is never shared with anyone. It is used to sign a message sent to any recipient or to decrypt a
/// message that was sent encrypted from any recipients.
///
/// In general it is discouraged to serialize and transfer secrets over a network, so you might be missing
/// some of those methods. The exception to this rule for compatibility is to support for deserializing
/// [WIF](https://en.bitcoin.it/wiki/Wallet_import_format) strings usual in BTC wallets.
#[wasm_bindgen(js_name = PrivateKey)]
#[derive(Clone)]
pub struct JsMPrivateKey {
    inner: MPrivateKey,
}

#[wasm_bindgen(js_class = PrivateKey)]
impl JsMPrivateKey {
    /// Converts a {@link SecpPrivateKey} into a multicipher {@link PrivateKey}.
    #[wasm_bindgen(js_name = fromSecp)]
    pub fn from_secp(sk: &JsSecpPrivateKey) -> Self {
        let inner = MPrivateKey::from(sk.inner().clone());
        Self { inner }
    }

    /// Calculates the public key the belongs to this private key.
    #[wasm_bindgen(js_name = publicKey)]
    pub fn public_key(&self) -> JsMPublicKey {
        JsMPublicKey::from(self.inner.public_key())
    }

    /// Calculates the signature of a message that can be then verified using {@link PublicKey.validate_ecdsa}
    #[wasm_bindgen(js_name = signEcdsa)]
    pub fn sign_ecdsa(&self, data: &[u8]) -> JsMSignature {
        let sig = self.inner.sign(data);
        JsMSignature::from(sig)
    }
}

impl From<MPrivateKey> for JsMPrivateKey {
    fn from(inner: MPrivateKey) -> Self {
        Self { inner }
    }
}

impl Wraps<MPrivateKey> for JsMPrivateKey {
    fn inner(&self) -> &MPrivateKey {
        &self.inner
    }
}

/// Secp256k1 private key
#[wasm_bindgen(js_name = SecpPrivateKey)]
#[derive(Clone)]
pub struct JsSecpPrivateKey {
    inner: SecpPrivateKey,
}

#[wasm_bindgen(js_class = SecpPrivateKey)]
impl JsSecpPrivateKey {
    /// Creates a {@link SecpPrivateKey} from a passphrase compatible with ark.io wallets.
    ///
    /// An Ark passphrase is a secret that must not be kept unencrypted in transit or in rest!
    #[wasm_bindgen(js_name = fromArkPassphrase)]
    pub fn from_ark_passphrase(phrase: &str) -> Result<JsSecpPrivateKey, JsValue> {
        let inner = SecpPrivateKey::from_ark_passphrase(phrase).map_err_to_js()?;
        Ok(Self { inner })
    }

    /// Creates a {@link SecpPrivateKey} from a WIF string compatible with BTC-related wallets. The
    /// second argument is a network name, that {@link validateNetworkName} accepts.
    ///
    /// A WIF is a secret that must not be kept unencrypted in transit or in rest!
    #[wasm_bindgen(js_name = fromWif)]
    pub fn from_wif(wif: &str, network: &str) -> Result<JsSecpPrivateKey, JsValue> {
        let network = Networks::by_name(network).map_err_to_js()?;
        let (inner, _bip178) = SecpPrivateKey::from_wif(wif, network).map_err_to_js()?;
        Ok(Self { inner })
    }

    /// Creates a WIF string compatible with BTC-related wallets. The second argument is a
    /// network name, that {@link validateNetworkName} accepts.
    ///
    /// This is a secret that must not be kept unencrypted in transit or in rest!
    #[wasm_bindgen(js_name = toWif)]
    pub fn to_wif(&self, network: &str) -> Result<String, JsValue> {
        let network = Networks::by_name(network).map_err_to_js()?;
        Ok(self.inner.to_wif(network.wif(), Bip178::Compressed))
    }

    /// Calculates the public key the belongs to this private key.
    #[wasm_bindgen(js_name = publicKey)]
    pub fn public_key(&self) -> JsSecpPublicKey {
        JsSecpPublicKey::from(self.inner.public_key())
    }

    /// Calculates the signature of a message that can be then verified using {@link SecpPublicKey.validate_ecdsa}
    #[wasm_bindgen(js_name = signEcdsa)]
    pub fn sign_ecdsa(&self, data: &[u8]) -> JsSecpSignature {
        let sig = self.inner.sign(data);
        JsSecpSignature::from(sig)
    }
}

impl From<SecpPrivateKey> for JsSecpPrivateKey {
    fn from(inner: SecpPrivateKey) -> Self {
        Self { inner }
    }
}

impl Wraps<SecpPrivateKey> for JsSecpPrivateKey {
    fn inner(&self) -> &SecpPrivateKey {
        &self.inner
    }
}
