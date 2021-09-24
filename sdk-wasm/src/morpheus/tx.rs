use super::*;

use iop_hydra_proto::txtype::{morpheus::Transaction, Aip29Transaction, CommonTransactionFields};
use iop_morpheus_proto::txtype::{
    MorpheusAsset, OperationAttempt, SignableOperation, SignableOperationAttempt,
    SignableOperationDetails, SignedOperation,
};

#[wasm_bindgen(js_name = MorpheusTxBuilder)]
pub struct JsMorpheusTxBuilder {}

#[wasm_bindgen(js_class = MorpheusTxBuilder)]
impl JsMorpheusTxBuilder {
    pub fn build(
        network_name: &str, morpheus_asset: JsValue, sender_pubkey: &JsSecpPublicKey, nonce: u64,
    ) -> Result<JsValue, JsValue> {
        let morpheus_asset: MorpheusAsset = morpheus_asset.into_serde().map_err_to_js()?;
        let common_fields = CommonTransactionFields {
            network: Networks::by_name(network_name).map_err_to_js()?,
            sender_public_key: sender_pubkey.inner().to_owned(),
            nonce,
            optional: Default::default(),
        };
        let morpheus_tx = Transaction::new(common_fields, morpheus_asset.operation_attempts);
        JsValue::from_serde(&morpheus_tx.to_data()).map_err_to_js()
    }
}

#[wasm_bindgen(js_name = MorpheusAssetBuilder)]
pub struct JsMorpheusAssetBuilder {
    op_attempts: Vec<OperationAttempt>,
}

#[wasm_bindgen(js_class = MorpheusAssetBuilder)]
impl JsMorpheusAssetBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsMorpheusAssetBuilder {
        JsMorpheusAssetBuilder { op_attempts: vec![] }
    }

    #[wasm_bindgen(js_name = addRegisterBeforeProof)]
    pub fn add_register_before_proof(&mut self, content_id: &str) {
        let before_proof =
            OperationAttempt::RegisterBeforeProof { content_id: content_id.to_owned() };
        self.op_attempts.push(before_proof);
    }

    #[wasm_bindgen(js_name = addSigned)]
    pub fn add_signed(&mut self, signed_operation: &JsMorpheusSignedOperation) {
        self.op_attempts.push(OperationAttempt::Signed(signed_operation.inner.to_owned()));
    }

    pub fn build(&self) -> Result<JsValue, JsValue> {
        let asset = MorpheusAsset::new(self.op_attempts.to_owned());
        JsValue::from_serde(&asset).map_err_to_js()
    }
}

#[wasm_bindgen(js_name = MorpheusSignableOperation)]
pub struct JsMorpheusSignableOperation {
    inner: SignableOperationAttempt,
}

#[wasm_bindgen(js_class = MorpheusSignableOperation)]
impl JsMorpheusSignableOperation {
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.inner).map_err_to_js()
    }
}

impl From<SignableOperationAttempt> for JsMorpheusSignableOperation {
    fn from(inner: SignableOperationAttempt) -> Self {
        Self { inner }
    }
}

impl Wraps<SignableOperationAttempt> for JsMorpheusSignableOperation {
    fn inner(&self) -> &SignableOperationAttempt {
        &self.inner
    }
}

#[wasm_bindgen(js_name = MorpheusOperationBuilder)]
pub struct JsMorpheusOperationBuilder {
    did: Did,
    last_tx_id: Option<String>,
}

#[wasm_bindgen(js_class = MorpheusOperationBuilder)]
impl JsMorpheusOperationBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(did: &str, last_tx_id: JsValue) -> Result<JsMorpheusOperationBuilder, JsValue> {
        let last_tx_id = last_tx_id.into_serde().map_err_to_js()?;
        Ok(JsMorpheusOperationBuilder { did: did.parse().map_err_to_js()?, last_tx_id })
    }

    #[wasm_bindgen(js_name = addKey)]
    pub fn add_key(
        &self, authentication: &str, expires_at_height: JsValue,
    ) -> Result<JsMorpheusSignableOperation, JsValue> {
        let auth = Authentication::from_str(authentication).map_err_to_js()?;
        let expires_at_height = expires_at_height.into_serde().map_err_to_js()?;
        let operation = SignableOperationDetails::AddKey { auth, expires_at_height };
        self.to_attempt(operation)
    }

    #[wasm_bindgen(js_name = revokeKey)]
    pub fn revoke_key(&self, authentication: &str) -> Result<JsMorpheusSignableOperation, JsValue> {
        let auth = Authentication::from_str(authentication).map_err_to_js()?;
        let operation = SignableOperationDetails::RevokeKey { auth };
        self.to_attempt(operation)
    }

    #[wasm_bindgen(js_name = addRight)]
    pub fn add_right(
        &self, authentication: &str, right: &str,
    ) -> Result<JsMorpheusSignableOperation, JsValue> {
        let auth = Authentication::from_str(authentication).map_err_to_js()?;
        let operation = SignableOperationDetails::AddRight { auth, right: right.to_owned() };
        self.to_attempt(operation)
    }

    #[wasm_bindgen(js_name = revokeRight)]
    pub fn revoke_right(
        &self, authentication: &str, right: &str,
    ) -> Result<JsMorpheusSignableOperation, JsValue> {
        let auth = Authentication::from_str(authentication).map_err_to_js()?;
        let operation = SignableOperationDetails::RevokeRight { auth, right: right.to_owned() };
        self.to_attempt(operation)
    }

    #[wasm_bindgen(js_name = tombstoneDid)]
    pub fn tombstone_did(&self) -> Result<JsMorpheusSignableOperation, JsValue> {
        let operation = SignableOperationDetails::TombstoneDid {};
        self.to_attempt(operation)
    }
}

impl JsMorpheusOperationBuilder {
    fn to_attempt(
        &self, operation: SignableOperationDetails,
    ) -> Result<JsMorpheusSignableOperation, JsValue> {
        let attempt = SignableOperationAttempt {
            did: self.did.to_owned(),
            last_tx_id: self.last_tx_id.to_owned(),
            operation,
        };
        Ok(attempt.into())
    }
}

#[wasm_bindgen(js_name = MorpheusOperationSigner)]
pub struct JsMorpheusOperationSigner {
    signables: Vec<SignableOperationAttempt>,
}

#[wasm_bindgen(js_class = MorpheusOperationSigner)]
impl JsMorpheusOperationSigner {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsMorpheusOperationSigner {
        Self { signables: vec![] }
    }

    pub fn add(&mut self, signable: &JsMorpheusSignableOperation) {
        self.signables.push(signable.inner().to_owned())
    }

    #[wasm_bindgen(js_name=signWithKey)]
    pub fn sign_with_key(
        &self, private_key: &JsMPrivateKey,
    ) -> Result<JsMorpheusSignedOperation, JsValue> {
        self.sign_inner(private_key.inner().to_owned())
    }

    pub fn sign(
        &self, public_key: JsMPublicKey, morpheus_private: &JsMorpheusPrivate,
    ) -> Result<JsMorpheusSignedOperation, JsValue> {
        let private_key =
            morpheus_private.inner().key_by_pk(&public_key.inner()).map_err_to_js()?;
        self.sign_inner(private_key.private_key())
    }

    fn sign_inner(&self, private_key: MPrivateKey) -> Result<JsMorpheusSignedOperation, JsValue> {
        let signable_ops = SignableOperation::new(self.signables.to_owned());
        let signer = PrivateKeySigner::new(private_key);
        let signed = signable_ops.sign(&signer).map_err_to_js()?;
        Ok(signed.into())
    }
}

#[wasm_bindgen(js_name = MorpheusSignedOperation)]
pub struct JsMorpheusSignedOperation {
    inner: SignedOperation,
}

#[wasm_bindgen(js_class = MorpheusSignedOperation)]
impl JsMorpheusSignedOperation {
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.inner).map_err_to_js()
    }
}

impl From<SignedOperation> for JsMorpheusSignedOperation {
    fn from(inner: SignedOperation) -> Self {
        Self { inner }
    }
}

impl Wraps<SignedOperation> for JsMorpheusSignedOperation {
    fn inner(&self) -> &SignedOperation {
        &self.inner
    }
}
