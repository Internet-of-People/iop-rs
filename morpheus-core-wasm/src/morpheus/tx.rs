use super::*;

use iop_hydra_proto::txtype::{
    morpheus::{
        OperationAttempt, SignableOperation, SignableOperationAttempt, SignableOperationDetails,
        Transaction,
    },
    Aip29Transaction, CommonTransactionFields,
};
use iop_morpheus_core::data::auth::Authentication;

// TODO use strict types all around this API
//      i.e. create JsSpecificType instead of using JsValue at many places
#[wasm_bindgen(js_name = MorpheusTxBuilder)]
#[derive(Clone)]
pub struct JsMorpheusTxBuilder {
    common_fields: CommonTransactionFields,
    op_attempts: Vec<OperationAttempt>,
}

#[wasm_bindgen(js_class = MorpheusTxBuilder)]
impl JsMorpheusTxBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(
        network_name: &str, sender_pubkey: &JsSecpPublicKey, nonce: u64,
    ) -> Result<JsMorpheusTxBuilder, JsValue> {
        let common_fields = CommonTransactionFields {
            network: Networks::by_name(network_name).map_err_to_js()?,
            sender_public_key: sender_pubkey.inner().to_owned(),
            nonce,
            optional: Default::default(),
        };
        let tx = JsMorpheusTxBuilder { common_fields, op_attempts: vec![] };
        Ok(tx.into())
    }

    #[wasm_bindgen(js_name = addRegisterBeforeProof)]
    pub fn add_register_before_proof(
        &self, content_id: &str,
    ) -> Result<JsMorpheusTxBuilder, JsValue> {
        let before_proof =
            OperationAttempt::RegisterBeforeProof { content_id: content_id.to_owned() };
        let mut result = self.clone();
        result.op_attempts.push(before_proof);
        Ok(result)
    }

    #[wasm_bindgen(js_name = addSigned)]
    pub fn add_signed(&self, signed_operation: &JsValue) -> Result<JsMorpheusTxBuilder, JsValue> {
        let signed_op = signed_operation.into_serde().map_err_to_js()?;
        let mut result = self.clone();
        result.op_attempts.push(OperationAttempt::Signed(signed_op));
        Ok(result)
    }

    pub fn build(&self) -> Result<JsValue, JsValue> {
        let morpheus_tx =
            Transaction::new(self.common_fields.to_owned(), self.op_attempts.to_owned());
        JsValue::from_serde(&morpheus_tx.to_data()).map_err_to_js()
    }
}

#[wasm_bindgen(js_name = MorpheusOperationBuilder)]
pub struct JsMorpheusOperationBuilder {
    did: String,
    last_tx_id: Option<String>,
}

#[wasm_bindgen(js_class = MorpheusOperationBuilder)]
impl JsMorpheusOperationBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(did: &str, last_tx_id: JsValue) -> Result<JsMorpheusOperationBuilder, JsValue> {
        let last_tx_id = last_tx_id.into_serde().map_err_to_js()?;
        Ok(JsMorpheusOperationBuilder { did: did.to_owned(), last_tx_id })
    }

    #[wasm_bindgen(js_name = addKey)]
    pub fn add_key(
        &self, authentication: &str, expires_at_height: JsValue,
    ) -> Result<JsValue, JsValue> {
        let auth = Authentication::from_str(authentication).map_err_to_js()?;
        let expires_at_height = expires_at_height.into_serde().map_err_to_js()?;
        let operation = SignableOperationDetails::AddKey { auth, expires_at_height };
        self.signable_attempt_js(operation)
    }

    #[wasm_bindgen(js_name = revokeKey)]
    pub fn revoke_key(&self, authentication: &str) -> Result<JsValue, JsValue> {
        let auth = Authentication::from_str(authentication).map_err_to_js()?;
        let operation = SignableOperationDetails::RevokeKey { auth };
        self.signable_attempt_js(operation)
    }

    #[wasm_bindgen(js_name = addRight)]
    pub fn add_right(&self, authentication: &str, right: &str) -> Result<JsValue, JsValue> {
        let auth = Authentication::from_str(authentication).map_err_to_js()?;
        let operation = SignableOperationDetails::AddRight { auth, right: right.to_owned() };
        self.signable_attempt_js(operation)
    }

    #[wasm_bindgen(js_name = revokeRight)]
    pub fn revoke_right(&self, authentication: &str, right: &str) -> Result<JsValue, JsValue> {
        let auth = Authentication::from_str(authentication).map_err_to_js()?;
        let operation = SignableOperationDetails::RevokeRight { auth, right: right.to_owned() };
        self.signable_attempt_js(operation)
    }

    #[wasm_bindgen(js_name = tombstoneDid)]
    pub fn tombstone_did(&self) -> Result<JsValue, JsValue> {
        let operation = SignableOperationDetails::TombstoneDid {};
        self.signable_attempt_js(operation)
    }
}

impl JsMorpheusOperationBuilder {
    fn signable_attempt_js(&self, operation: SignableOperationDetails) -> Result<JsValue, JsValue> {
        let signable_op = SignableOperationAttempt {
            did: self.did.to_owned(),
            last_tx_id: self.last_tx_id.to_owned(),
            operation,
        };
        let signable_op_js = JsValue::from_serde(&signable_op).map_err_to_js()?;
        Ok(signable_op_js)
    }
}

#[wasm_bindgen(js_name = signMorpheusOperations)]
pub fn sign_morpheus_operations(
    operations: &JsValue, private_key: &JsMPrivateKey,
) -> Result<JsValue, JsValue> {
    let signables = operations.into_serde().map_err_to_js()?;
    let signable_ops = SignableOperation::new(signables);
    let signer = PrivateKeySigner::new(private_key.inner().to_owned());
    let signed_ops = signable_ops.sign(&signer).map_err_to_js()?;
    JsValue::from_serde(&signed_ops).map_err_to_js()
}
