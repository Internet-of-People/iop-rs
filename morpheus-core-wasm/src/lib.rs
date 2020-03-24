use serde_json::Value;
use wasm_bindgen::prelude::*;

use keyvault::PublicKey as KeyVaultPublicKey;
use keyvault_wasm::*;
use morpheus_core::{
    crypto::sign::{PrivateKeySigner, Signable, Signed, SyncSigner},
    data::{
        auth::Authentication,
        claim::{WitnessRequest, WitnessStatement},
        did::Did,
        diddoc::BlockHeight,
        present::ClaimPresentation,
        validation::{ValidationIssue, ValidationResult},
    },
    vault::{InMemoryDidVault, SyncDidVault},
};

#[wasm_bindgen(js_name = SignedBytes)]
pub struct JsSignedBytes {
    inner: Signed<Box<[u8]>>,
}

#[wasm_bindgen(js_class = SignedBytes)]
impl JsSignedBytes {
    #[wasm_bindgen(constructor)]
    pub fn new(
        public_key: &JsPublicKey, content: &[u8], signature: &JsSignature,
    ) -> Result<JsSignedBytes, JsValue> {
        let inner = Signed::new(
            public_key.inner().to_owned(),
            content.to_owned().into_boxed_slice(),
            signature.inner().to_owned(),
        );
        Ok(Self { inner })
    }

    #[wasm_bindgen(getter, js_name = publicKey)]
    pub fn public_key(&self) -> JsPublicKey {
        JsPublicKey::from(self.inner.public_key().to_owned())
    }

    #[wasm_bindgen(getter)]
    pub fn content(&self) -> Box<[u8]> {
        self.inner.content().clone()
    }

    #[wasm_bindgen(getter)]
    pub fn signature(&self) -> JsSignature {
        JsSignature::from(self.inner.signature().to_owned())
    }

    pub fn validate(&self) -> Result<bool, JsValue> {
        let content = self.inner.content().content_to_sign().map_err(err_to_js)?;
        Ok(self.inner.public_key().verify(content, &self.inner.signature()))
    }
}

impl From<Signed<Box<[u8]>>> for JsSignedBytes {
    fn from(inner: Signed<Box<[u8]>>) -> Self {
        Self { inner }
    }
}

impl Wraps<Signed<Box<[u8]>>> for JsSignedBytes {
    fn inner(&self) -> &Signed<Box<[u8]>> {
        &self.inner
    }
}

#[wasm_bindgen(js_name = SignedJson)]
pub struct JsSignedJson {
    inner: Signed<serde_json::Value>,
}

#[wasm_bindgen(js_class = SignedJson)]
impl JsSignedJson {
    #[wasm_bindgen(constructor)]
    pub fn new(
        public_key: &JsPublicKey, content: &JsValue, signature: &JsSignature,
    ) -> Result<JsSignedJson, JsValue> {
        let inner = Signed::new(
            public_key.inner().to_owned(),
            content.into_serde().map_err(err_to_js)?,
            signature.inner().to_owned(),
        );
        Ok(Self { inner })
    }

    #[wasm_bindgen(getter, js_name = publicKey)]
    pub fn public_key(&self) -> JsPublicKey {
        JsPublicKey::from(self.inner.public_key().to_owned())
    }

    #[wasm_bindgen(getter)]
    pub fn content(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(self.inner.content()).map_err(err_to_js)
    }

    #[wasm_bindgen(getter)]
    pub fn signature(&self) -> JsSignature {
        JsSignature::from(self.inner.signature().to_owned())
    }

    pub fn validate(&self) -> bool {
        match self.inner.content().content_to_sign() {
            Ok(content) => self.inner.public_key().verify(content, &self.inner.signature()),
            Err(_) => return false,
        }
    }

    #[wasm_bindgen(js_name = validateWithKeyId)]
    pub fn validate_with_keyid(&self, signer_id: &JsKeyId) -> bool {
        self.inner.public_key().validate_id(&signer_id.inner()) && self.validate()
    }

    // TODO return red/yellow/green instead of bool
    #[wasm_bindgen(js_name = validateWithDid)]
    pub fn validate_with_did(
        &self, did_doc_str: &str, from_height_inc: Option<BlockHeight>,
        until_height_exc: Option<BlockHeight>,
    ) -> Result<JsValue, JsValue> {
        let did_doc = serde_json::from_str(did_doc_str).map_err(err_to_js)?;
        let result = self
            .inner
            .validate_with_did_doc(&did_doc, from_height_inc, until_height_exc)
            .map_err(err_to_js)?;
        Ok(JsValidationResult { inner: result }.into())
    }
}

impl From<Signed<serde_json::Value>> for JsSignedJson {
    fn from(inner: Signed<Value>) -> Self {
        Self { inner }
    }
}

impl Wraps<Signed<serde_json::Value>> for JsSignedJson {
    fn inner(&self) -> &Signed<Value> {
        &self.inner
    }
}

#[wasm_bindgen(js_name = ValidationIssue)]
pub struct JsValidationIssue {
    inner: ValidationIssue,
}

#[wasm_bindgen(js_class = ValidationIssue)]
impl JsValidationIssue {
    #[wasm_bindgen(getter)]
    pub fn code(&self) -> u32 {
        self.inner.code()
    }

    #[wasm_bindgen(getter)]
    pub fn severity(&self) -> String {
        self.inner.severity().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn reason(&self) -> String {
        self.inner.reason().to_string()
    }
}

impl From<ValidationIssue> for JsValidationIssue {
    fn from(inner: ValidationIssue) -> Self {
        Self { inner }
    }
}

impl Wraps<ValidationIssue> for JsValidationIssue {
    fn inner(&self) -> &ValidationIssue {
        &self.inner
    }
}

#[wasm_bindgen(js_name = ValidationResult)]
pub struct JsValidationResult {
    inner: ValidationResult,
}

#[wasm_bindgen(js_class = ValidationResult)]
impl JsValidationResult {
    #[wasm_bindgen(getter)]
    pub fn status(&self) -> String {
        self.inner.status().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn messages(&self) -> Box<[JsValue]> {
        let msgs = self
            .inner
            .issues()
            .iter()
            .map(|issue| JsValidationIssue { inner: issue.to_owned() }.into())
            .collect::<Vec<_>>();
        msgs.into_boxed_slice()
    }
}

impl From<ValidationResult> for JsValidationResult {
    fn from(inner: ValidationResult) -> Self {
        Self { inner }
    }
}

impl Wraps<ValidationResult> for JsValidationResult {
    fn inner(&self) -> &ValidationResult {
        &self.inner
    }
}

#[wasm_bindgen(js_name = Did)]
pub struct JsDid {
    inner: Did,
}

#[wasm_bindgen(js_class = Did)]
impl JsDid {
    #[wasm_bindgen(constructor)]
    pub fn new(did_str: &str) -> Result<JsDid, JsValue> {
        let inner: Did = did_str.parse().map_err(err_to_js)?;
        Ok(Self { inner })
    }

    #[wasm_bindgen(js_name = fromKeyId)]
    pub fn from_key_id(key_id: &JsKeyId) -> Self {
        Did::from(key_id.inner()).into()
    }

    #[wasm_bindgen(js_name = defaultKeyId)]
    pub fn default_key_id(&self) -> JsKeyId {
        JsKeyId::from(self.inner.default_key_id())
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
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

#[wasm_bindgen(js_name = Vault)]
pub struct JsVault {
    inner: InMemoryDidVault,
}

#[wasm_bindgen(js_class = Vault)]
impl JsVault {
    #[wasm_bindgen(constructor)]
    pub fn new(seed_phrase: &str) -> Result<JsVault, JsValue> {
        let seed = keyvault::Seed::from_bip39(seed_phrase).map_err(err_to_js)?;
        let vault = InMemoryDidVault::new(seed);
        Ok(Self { inner: vault })
    }

    pub fn serialize(&self) -> Result<String, JsValue> {
        serde_json::to_string_pretty(&self.inner).map_err(err_to_js)
    }

    pub fn deserialize(from: &str) -> Result<JsVault, JsValue> {
        let vault = serde_json::from_str(&from).map_err(err_to_js)?;
        Ok(Self { inner: vault })
    }

    #[wasm_bindgen(js_name = keyIds)]
    pub fn key_ids(&self) -> Result<Box<[JsValue]>, JsValue> {
        let key_ids = self
            .inner
            .key_ids()
            .map_err(err_to_js)?
            .iter()
            .map(|keyid| JsKeyId::from(keyid.to_owned()).into())
            .collect::<Vec<_>>();
        Ok(key_ids.into_boxed_slice())
    }

    pub fn dids(&self) -> Result<Box<[JsValue]>, JsValue> {
        let dids = self
            .inner
            .dids()
            .map_err(err_to_js)?
            .iter()
            .map(|did| JsDid::from(did.to_owned()).into())
            .collect::<Vec<_>>();
        Ok(dids.into_boxed_slice())
    }

    #[wasm_bindgen(js_name = activeDid)]
    pub fn active_did(&self) -> Result<Option<JsDid>, JsValue> {
        let active_did = self.inner.get_active().map_err(err_to_js)?;
        Ok(active_did.map(|did| did.into()))
    }

    #[wasm_bindgen(js_name = createDid)]
    pub fn create_did(&mut self) -> Result<JsDid, JsValue> {
        let key = self.inner.create(None).map_err(err_to_js)?;
        Ok(key.did().into())
    }

    #[wasm_bindgen(js_name = signWitnessRequest)]
    pub fn sign_witness_request(
        &self, key_id: &JsKeyId, js_req: &JsValue,
    ) -> Result<JsSignedJson, JsValue> {
        let signer = self.signer_by_auth(key_id)?;
        let request: WitnessRequest = js_req.into_serde().map_err(err_to_js)?;
        let signed_request = signer.sign_witness_request(request).map_err(err_to_js)?;
        Self::to_signed_json(signed_request)
    }

    #[wasm_bindgen(js_name = signWitnessStatement)]
    pub fn sign_witness_statement(
        &self, key_id: &JsKeyId, js_stmt: &JsValue,
    ) -> Result<JsSignedJson, JsValue> {
        let signer = self.signer_by_auth(key_id)?;
        let statement: WitnessStatement = js_stmt.into_serde().map_err(err_to_js)?;
        let signed_statement = signer.sign_witness_statement(statement).map_err(err_to_js)?;
        Self::to_signed_json(signed_statement)
    }

    #[wasm_bindgen(js_name = signClaimPresentation)]
    pub fn sign_claim_presentation(
        &self, key_id: &JsKeyId, js_presentation: &JsValue,
    ) -> Result<JsSignedJson, JsValue> {
        let signer = self.signer_by_auth(key_id)?;
        let presentation: ClaimPresentation = js_presentation.into_serde().map_err(err_to_js)?;
        let signed_presentation =
            signer.sign_claim_presentation(presentation).map_err(err_to_js)?;
        Self::to_signed_json(signed_presentation)
    }

    // TODO reconsider typing and strictness of 'operations' argument
    #[wasm_bindgen(js_name = signDidOperations)]
    pub fn sign_did_operations(
        &self, key_id: &JsKeyId, js_operations: Box<[u8]>,
    ) -> Result<JsSignedBytes, JsValue> {
        let signer = self.signer_by_auth(key_id)?;
        let (public_key, signature) = signer.sign(js_operations.as_ref()).map_err(err_to_js)?;
        let signed_bytes = Signed::new(public_key, js_operations, signature);
        Ok(JsSignedBytes::from(signed_bytes))
    }
}

impl JsVault {
    fn signer_by_auth(&self, key_id: &JsKeyId) -> Result<PrivateKeySigner, JsValue> {
        let signer = self
            .inner
            .signer_by_auth(&Authentication::KeyId(key_id.inner().to_owned()))
            .map_err(err_to_js)?;
        Ok(signer)
    }

    fn to_signed_json<T: Signable>(signed: Signed<T>) -> Result<JsSignedJson, JsValue> {
        let signed_json = Signed::new(
            signed.public_key().to_owned(),
            serde_json::to_value(signed.content()).map_err(err_to_js)?,
            signed.signature().to_owned(),
        );
        Ok(signed_json.into())
    }
}
