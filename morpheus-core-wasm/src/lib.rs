use wasm_bindgen::prelude::*;

use keyvault::PublicKey as KeyVaultPublicKey;
use morpheus_core::crypto::sign::Signed;
use morpheus_core::data::diddoc::BlockHeight;
use morpheus_core::data::validation::ValidationStatus;

fn err_to_js<E: ToString>(e: E) -> JsValue {
    JsValue::from(e.to_string())
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct KeyId {
    inner: keyvault::multicipher::MKeyId,
}

#[wasm_bindgen]
impl KeyId {
    #[wasm_bindgen(constructor)]
    pub fn new(key_id_str: &str) -> Result<KeyId, JsValue> {
        let inner: keyvault::multicipher::MKeyId = key_id_str.parse().map_err(err_to_js)?;
        Ok(Self { inner })
    }

    #[wasm_bindgen]
    pub fn prefix() -> String {
        keyvault::multicipher::MKeyId::PREFIX.to_string()
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.inner.to_string()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct PublicKey {
    inner: keyvault::multicipher::MPublicKey,
}

#[wasm_bindgen]
impl PublicKey {
    #[wasm_bindgen(constructor)]
    pub fn new(pub_key_str: &str) -> Result<PublicKey, JsValue> {
        let inner: keyvault::multicipher::MPublicKey = pub_key_str.parse().map_err(err_to_js)?;
        Ok(Self { inner })
    }

    #[wasm_bindgen]
    pub fn prefix() -> String {
        keyvault::multicipher::MPublicKey::PREFIX.to_string()
    }

    #[wasm_bindgen(js_name = keyId)]
    pub fn key_id(&self) -> KeyId {
        KeyId { inner: self.inner.key_id() }
    }

    #[wasm_bindgen(js_name = validateId)]
    pub fn validate_id(&self, key_id: &KeyId) -> bool {
        self.inner.validate_id(&key_id.inner)
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.inner.to_string()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Signature {
    inner: keyvault::multicipher::MSignature,
}

#[wasm_bindgen]
impl Signature {
    #[wasm_bindgen(constructor)]
    pub fn new(sign_str: &str) -> Result<Signature, JsValue> {
        let inner: keyvault::multicipher::MSignature = sign_str.parse().map_err(err_to_js)?;
        Ok(Self { inner })
    }

    #[wasm_bindgen]
    pub fn prefix() -> String {
        keyvault::multicipher::MSignature::PREFIX.to_string()
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.inner.to_string()
    }
}

#[wasm_bindgen]
pub struct SignedString {
    inner: Signed<String>,
}

#[wasm_bindgen]
impl SignedString {
    #[wasm_bindgen(constructor)]
    pub fn new(public_key: &PublicKey, content: &str, signature: &Signature) -> Self {
        let inner = Signed::new(
            public_key.inner.to_owned(),
            content.to_owned(),
            signature.inner.to_owned(),
        );
        Self { inner }
    }

    #[wasm_bindgen(getter, js_name = publicKey)]
    pub fn public_key(&self) -> PublicKey {
        PublicKey { inner: self.inner.public_key().to_owned() }
    }

    #[wasm_bindgen(getter)]
    pub fn content(&self) -> String {
        self.inner.content().to_owned()
    }

    #[wasm_bindgen(getter)]
    pub fn signature(&self) -> Signature {
        Signature { inner: self.inner.signature().to_owned() }
    }

    pub fn validate(&self) -> bool {
        self.inner.public_key().verify(self.inner.content(), &self.inner.signature())
    }

    #[wasm_bindgen(js_name = validateWithKeyId)]
    pub fn validate_with_keyid(&self, signer_id: &KeyId) -> bool {
        self.inner.public_key().validate_id(&signer_id.inner) && self.validate()
    }

    // TODO return red/yellow/green instead of bool
    #[wasm_bindgen(js_name = validateWithDid)]
    pub fn validate_with_did(
        &self, did_doc_str: &str, from_height_inc: Option<BlockHeight>,
        until_height_exc: Option<BlockHeight>,
    ) -> Result<bool, JsValue> {
        let did_doc = serde_json::from_str(did_doc_str).map_err(err_to_js)?;
        let status = self
            .inner
            .validate_with_did_doc(&did_doc, from_height_inc, until_height_exc)
            .map_err(err_to_js)?;
        Ok(status == ValidationStatus::Valid)
    }
}
