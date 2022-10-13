use super::*;

use iop_hydra_proto::txtype::{morpheus::Transaction, Aip29Transaction, CommonTransactionFields};
use iop_morpheus_proto::txtype::{
    MorpheusAsset, OperationAttempt, SignableOperation, SignableOperationAttempt,
    SignableOperationDetails, SignedOperation,
};

/// Builder for SSI Hydra transactions.
#[wasm_bindgen(js_name = MorpheusTxBuilder)]
pub struct JsMorpheusTxBuilder {}

#[wasm_bindgen(js_class = MorpheusTxBuilder)]
impl JsMorpheusTxBuilder {
    /// Creates an unsigned SSI transaction on a given network from the {@link IMorpheusAsset}.
    ///
    /// The nonce of the sender needs to be known in advance and the next transaction must be 1 above the one of the last transaction
    /// made by the sender on-chain.
    ///
    /// Vendor field is a public memo attached to the transaction. The fee can be manually overriden, or the defaults will be
    /// calculated based on the size of the serialized transaction size and some offset based on the transaction type.
    pub fn build(
        network_name: &str, morpheus_asset: &JsValue, sender_pubkey: &JsSecpPublicKey, nonce: u64,
        vendor_field: Option<String>, manual_fee: Option<u64>,
    ) -> Result<JsValue, JsValue> {
        let morpheus_asset: MorpheusAsset = from_value(morpheus_asset.clone())?;
        let common_fields = CommonTransactionFields {
            network: Networks::by_name(network_name).map_err_to_js()?,
            sender_public_key: sender_pubkey.inner().to_owned(),
            nonce,
            optional: OptionalTransactionFields { amount: 0, vendor_field, manual_fee },
        };
        let morpheus_tx = Transaction::new(common_fields, morpheus_asset.operation_attempts);
        let res = to_value(&morpheus_tx.to_data())?;
        Ok(res)
    }
}

/// Builder for a {@link IMorpheusAsset}.
///
/// @see MorpheusTxBuilder, HydraSigner, HydraPrivate.signHydraTransaction
#[wasm_bindgen(js_name = MorpheusAssetBuilder)]
pub struct JsMorpheusAssetBuilder {
    op_attempts: Vec<OperationAttempt>,
}

#[wasm_bindgen(js_class = MorpheusAssetBuilder)]
impl JsMorpheusAssetBuilder {
    /// Creates a new instance. Assets are not dependent on the actual network they will be sent into in an SSI transaction.
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsMorpheusAssetBuilder {
        JsMorpheusAssetBuilder { op_attempts: vec![] }
    }

    /// Adds an operation that registers a proof of existence (before proof) for a given content.
    ///
    /// @see digestJson
    #[wasm_bindgen(js_name = addRegisterBeforeProof)]
    pub fn add_register_before_proof(&mut self, content_id: &str) {
        let before_proof =
            OperationAttempt::RegisterBeforeProof { content_id: content_id.to_owned() };
        self.op_attempts.push(before_proof);
    }

    /// Adds a set of operations, which alter DID documents, signed already with a key that has update rights on the DIDs being
    /// modified.
    ///
    /// @see MorpheusSignedOperation
    #[wasm_bindgen(js_name = addSigned)]
    pub fn add_signed(&mut self, signed_operation: &JsMorpheusSignedOperation) {
        self.op_attempts.push(OperationAttempt::Signed(signed_operation.inner.to_owned()));
    }

    /// Creates the serialized asset that can be added into an SSI transaction.
    ///
    /// @see MorpheusTxBuilder
    pub fn build(&self) -> Result<JsValue, JsValue> {
        let asset = MorpheusAsset::new(self.op_attempts.to_owned());
        let res = to_value(&asset)?;
        Ok(res)
    }
}

/// An object representing a single SSI operation on a single DID. This operation is not yet signed by a key with update
/// rights on the DID document, and therefore needs to be added to a {@link MorpheusOperationSigner}
#[wasm_bindgen(js_name = MorpheusSignableOperation)]
pub struct JsMorpheusSignableOperation {
    inner: SignableOperationAttempt,
}

#[wasm_bindgen(js_class = MorpheusSignableOperation)]
impl JsMorpheusSignableOperation {
    /// Deserializes a single unsigned SSI operation from a JSON.
    #[wasm_bindgen(constructor)]
    pub fn new(json: &JsValue) -> Result<JsMorpheusSignableOperation, JsValue> {
        let inner: SignableOperationAttempt = from_value(json.clone())?;
        Ok(JsMorpheusSignableOperation { inner })
    }

    /// Serializes a single unsigned SSI operation into a JSON.
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        to_value(&self.inner).map_err_to_js()
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

/// Builder for operations on a given DID. These operations can be later added to a {@link MorpheusOperationSigner} even for
/// different DIDs, so the operations can be signed by a multicipher {@link PrivateKey} that has update rights on these DIDs.
#[wasm_bindgen(js_name = MorpheusOperationBuilder)]
pub struct JsMorpheusOperationBuilder {
    did: Did,
    last_tx_id: Option<String>,
}

#[wasm_bindgen(js_class = MorpheusOperationBuilder)]
impl JsMorpheusOperationBuilder {
    /// Create an operation builder acting on a given state of a given DID. The last transaction ID that successfully altered
    /// the DID on-chain can be queried on the blockchain that the SSI transaction will be sent to. If no transactions modified the
    /// implicit DID document yet, this parameter must be `null`.
    #[wasm_bindgen(constructor)]
    pub fn new(did: &str, last_tx_id: &JsValue) -> Result<JsMorpheusOperationBuilder, JsValue> {
        let last_tx_id = from_value(last_tx_id.clone())?;
        let did = did.parse().map_err_to_js()?;
        Ok(JsMorpheusOperationBuilder { did, last_tx_id })
    }

    /// Create an add key operation. The key can be a {@link KeyId} or a {@link PublicKey} serialized into a string. The expiration can
    /// be left `null`, or it can be a block height, when the key is automatically revoked on-chain without a new transaction sent in.
    ///
    /// The same key cannot be added when it has not been revoked or before has expired, even if one addition uses an identifier of
    /// the key, and the other addition uses the public key. But the key can be re-added after it has expired or been revoked from the
    /// DID.
    #[wasm_bindgen(js_name = addKey)]
    pub fn add_key(
        &self, authentication: &str, expires_at_height: &JsValue,
    ) -> Result<JsMorpheusSignableOperation, JsValue> {
        let auth = Authentication::from_str(authentication).map_err_to_js()?;
        let expires_at_height = from_value(expires_at_height.clone())?;
        let operation = SignableOperationDetails::AddKey { auth, expires_at_height };
        self.to_attempt(operation)
    }

    /// Create a revoke key operation. A key cannot be revoked if it was not added or has already been revoked or has expired.
    #[wasm_bindgen(js_name = revokeKey)]
    pub fn revoke_key(&self, authentication: &str) -> Result<JsMorpheusSignableOperation, JsValue> {
        let auth = Authentication::from_str(authentication).map_err_to_js()?;
        let operation = SignableOperationDetails::RevokeKey { auth };
        self.to_attempt(operation)
    }

    /// Add a given right to a key. 'impersonate' or 'update' are the only choices yet. Cannot add a right to a key that has not yet
    /// been added to the DID document. Cannot add a right if it was already granted to the key on this DID.
    ///
    /// @see SystemRights
    #[wasm_bindgen(js_name = addRight)]
    pub fn add_right(
        &self, authentication: &str, right: &str,
    ) -> Result<JsMorpheusSignableOperation, JsValue> {
        let auth = Authentication::from_str(authentication).map_err_to_js()?;
        let operation = SignableOperationDetails::AddRight { auth, right: right.to_owned() };
        self.to_attempt(operation)
    }

    /// Revoke a given right from a key. 'impersonate' or 'update' are the only choices yet. Cannot revoke a right to a key that has
    /// not yet been added to the DID document. Cannot revoke a right if it was not yet granted to the key on this DID.
    ///
    /// @see SystemRights
    #[wasm_bindgen(js_name = revokeRight)]
    pub fn revoke_right(
        &self, authentication: &str, right: &str,
    ) -> Result<JsMorpheusSignableOperation, JsValue> {
        let auth = Authentication::from_str(authentication).map_err_to_js()?;
        let operation = SignableOperationDetails::RevokeRight { auth, right: right.to_owned() };
        self.to_attempt(operation)
    }

    /// Tombstone a DID. All keys and rights are effectively revoked, and the DID cannot be altered any further.
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

/// Builder object for collecting SSI operations into a bundle signed by a single multicipher {@link PrivateKey} that has update rights
/// on all DIDs being altered in those operations.
#[wasm_bindgen(js_name = MorpheusOperationSigner)]
pub struct JsMorpheusOperationSigner {
    signables: Vec<SignableOperationAttempt>,
}

#[wasm_bindgen(js_class = MorpheusOperationSigner)]
impl JsMorpheusOperationSigner {
    /// Creates a new {@link MorpheusOperationSigner}.
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsMorpheusOperationSigner {
        Self { signables: vec![] }
    }

    /// Adds a single SSI operation into the bundle that will be signed.
    ///
    /// @see sign, sign_with_key, MorpheusOperationBuilder, MorpheusSignableOperation.new
    pub fn add(&mut self, signable: &JsMorpheusSignableOperation) {
        self.signables.push(signable.inner().to_owned())
    }

    /// Sign the bundle of SSI operations with the provided {@link PrivateKey}.
    ///
    /// Returns a {@link MorpheusSignedOperation} that can be provided to {@link MorpheusAssetBuilder.addSigned}.
    #[wasm_bindgen(js_name=signWithKey)]
    pub fn sign_with_key(
        &self, private_key: &JsMPrivateKey,
    ) -> Result<JsMorpheusSignedOperation, JsValue> {
        self.sign_inner(private_key.inner().to_owned())
    }

    /// A convenience method to sign the bundle of SSI operations with a {@link PublicKey} from the vault.
    ///
    /// Returns a {@link MorpheusSignedOperation} that can be provided to {@link MorpheusAssetBuilder.addSigned}.
    ///
    /// @see MorpheusPrivate, MorpheusPrivate.key_by_pk
    pub fn sign(
        &self, public_key: JsMPublicKey, morpheus_private: &JsMorpheusPrivate,
    ) -> Result<JsMorpheusSignedOperation, JsValue> {
        let private_key = morpheus_private.inner().key_by_pk(public_key.inner()).map_err_to_js()?;
        self.sign_inner(private_key.private_key())
    }

    /// A convenience method to sign the bundle of SSI operations with a {@link KeyId} from the vault.
    ///
    /// Returns a {@link MorpheusSignedOperation} that can be provided to {@link MorpheusAssetBuilder.addSigned}.
    ///
    /// @see MorpheusPrivate, MorpheusPrivate.key_by_id
    #[wasm_bindgen(js_name=signWithId)]
    pub fn sign_with_id(
        &self, key_id: JsMKeyId, morpheus_private: &JsMorpheusPrivate,
    ) -> Result<JsMorpheusSignedOperation, JsValue> {
        let public_key =
            morpheus_private.inner().public().key_by_id(key_id.inner()).map_err_to_js()?;
        let private_key = morpheus_private.inner().key_by_pk(&public_key).map_err_to_js()?;
        self.sign_inner(private_key.private_key())
    }

    fn sign_inner(&self, private_key: MPrivateKey) -> Result<JsMorpheusSignedOperation, JsValue> {
        let signable_ops = SignableOperation::new(self.signables.to_owned());
        let signer = PrivateKeySigner::new(private_key);
        let signed = signable_ops.sign(&signer).map_err_to_js()?;
        Ok(signed.into())
    }
}

/// A set of SSI operations already signed by a key that had update rights on all DIDs altered by the operations.
#[wasm_bindgen(js_name = MorpheusSignedOperation)]
pub struct JsMorpheusSignedOperation {
    inner: SignedOperation,
}

#[wasm_bindgen(js_class = MorpheusSignedOperation)]
impl JsMorpheusSignedOperation {
    /// Deserializes a set of signed SSI operations from a JSON.
    #[wasm_bindgen(constructor)]
    pub fn new(json: &JsValue) -> Result<JsMorpheusSignedOperation, JsValue> {
        let inner: SignedOperation = from_value(json.clone())?;
        Ok(JsMorpheusSignedOperation { inner })
    }

    /// Serializes a set of signed SSI operations into a JSON.
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        to_value(&self.inner).map_err_to_js()
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
