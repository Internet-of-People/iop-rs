use super::*;

/// Private keys of the Morpheus subtree in a vault.
///
/// @see MorpheusPlugin.priv
#[wasm_bindgen(js_name = MorpheusPrivate)]
pub struct JsMorpheusPrivate {
    inner: MorpheusPrivate,
}

#[wasm_bindgen(js_class = MorpheusPrivate)]
impl JsMorpheusPrivate {
    /// Access to the public keys of the subtree. Same as {@link MorpheusPlugin.pub} would return.
    #[wasm_bindgen(getter = pub)]
    pub fn public(&self) -> JsMorpheusPublic {
        let inner = self.inner.public();
        JsMorpheusPublic::from(inner)
    }

    /// Accessor for the BIP32 path of the Morpheus subtree.
    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.node().path().to_string()
    }

    /// There can be several usages of DIDs differentiated inside the vault invisible externally, e.g. on a blockchain.
    /// Each represents a separate subtree under the Morpheus subtree in the vault.
    ///
    /// Use 'persona', 'device', 'group' or 'resource' in singular as a parameter.
    pub fn kind(&self, did_kind: &str) -> Result<JsMorpheusPrivateKind, JsValue> {
        let did_kind: DidKind = did_kind.parse().map_err_to_js()?;
        self.kind_impl(did_kind)
    }

    /// Alias for {@link kind('persona')}
    #[wasm_bindgen(getter)]
    pub fn personas(&self) -> Result<JsMorpheusPrivateKind, JsValue> {
        self.kind_impl(DidKind::Persona)
    }

    /// Alias for {@link kind('device')}
    #[wasm_bindgen(getter)]
    pub fn devices(&self) -> Result<JsMorpheusPrivateKind, JsValue> {
        self.kind_impl(DidKind::Device)
    }

    /// Alias for {@link kind('group')}
    #[wasm_bindgen(getter)]
    pub fn groups(&self) -> Result<JsMorpheusPrivateKind, JsValue> {
        self.kind_impl(DidKind::Group)
    }

    /// Alias for {@link kind('resource')}
    #[wasm_bindgen(getter)]
    pub fn resources(&self) -> Result<JsMorpheusPrivateKind, JsValue> {
        self.kind_impl(DidKind::Resource)
    }

    /// Finds the {@link MorpheusPrivateKey} that belongs to the given multicipher {@link PublicKey}. You can check the DID kind or
    /// index of the key or get the actual {@link PrivateKey} from the returned object.
    ///
    /// An error will be thrown if the public key has never been used yet in this vault.
    #[wasm_bindgen(js_name = keyByPublicKey)]
    pub fn key_by_pk(&self, pk: &JsMPublicKey) -> Result<JsMorpheusPrivateKey, JsValue> {
        let inner = self.inner.key_by_pk(pk.inner()).map_err_to_js()?;
        Ok(JsMorpheusPrivateKey::from(inner))
    }

    /// Finds the {@link MorpheusPrivateKey} that belongs to the given multicipher {@link KeyId}. You can check the DID kind or
    /// index of the key or get the actual {@link PrivateKey} from the returned object.
    ///
    /// An error will be thrown if the key identifier has never been used yet in this vault.
    #[wasm_bindgen(js_name = keyById)]
    pub fn key_by_id(&self, id: &JsMKeyId) -> Result<JsMorpheusPrivateKey, JsValue> {
        let pk = self.inner.public().key_by_id(id.inner()).map_err_to_js()?;
        let js_pk = JsMPublicKey::from(pk);
        self.key_by_pk(&js_pk)
    }

    /// Signs some binary payload with a private key that belongs to the given multicipher {@link KeyId}.
    ///
    /// The returned {@link SignedBytes} has separate properties for signature and public key.
    ///
    /// Note, that usually it is usually a bad security to let the user sign a binary content that was not reviewed by the user on a
    /// trusted user interface **before** serialization into that binary format.
    ///
    /// @see keyById
    #[wasm_bindgen(js_name = signDidOperations)]
    pub fn sign_did_operations(
        &self, id: &JsMKeyId, message: &[u8],
    ) -> Result<JsSignedBytes, JsValue> {
        let signer = self.create_signer(id)?;
        let (public_key, signature) = signer.sign(message).map_err_to_js()?;

        let js_pk = JsMPublicKey::from(public_key);
        let js_sig = JsMSignature::from(signature);
        JsSignedBytes::new(&js_pk, message, &js_sig)
    }

    /// Signs a witness request for a verifiable claim with a private key that belongs to the given multicipher {@link KeyId}. An error
    /// will be thrown if the JSON does not conform to the schema of a witness request.
    ///
    /// The returned {@link SignedJson} has separate properties for signature and public key, but can be serialized into a JSON format
    /// widely used in the IOP Stack™.
    ///
    /// @see keyById
    #[wasm_bindgen(js_name = signWitnessRequest)]
    pub fn sign_witness_request(
        &self, id: &JsMKeyId, js_req: &JsValue,
    ) -> Result<JsSignedJson, JsValue> {
        let signer = self.create_signer(id)?;
        let request: WitnessRequest = js_req.into_serde().map_err(err_to_js)?;
        let signed_request = signer.sign_witness_request(request).map_err(err_to_js)?;

        into_signed_json(signed_request)
    }

    /// Signs a witness statemet for a verifiable claim with a private key that belongs to the given multicipher {@link KeyId}. An
    /// error will be thrown if the JSON does not conform to the schema of a witness statement.
    ///
    /// The returned {@link SignedJson} has separate properties for signature and public key, but can be serialized into a JSON format
    /// widely used in the IOP Stack™.
    ///
    /// @see keyById
    #[wasm_bindgen(js_name = signWitnessStatement)]
    pub fn sign_witness_statement(
        &self, id: &JsMKeyId, js_stmt: &JsValue,
    ) -> Result<JsSignedJson, JsValue> {
        let signer = self.create_signer(id)?;
        let statement: WitnessStatement = js_stmt.into_serde().map_err(err_to_js)?;
        let signed_statement = signer.sign_witness_statement(statement).map_err(err_to_js)?;

        into_signed_json(signed_statement)
    }

    /// Signs a claim presentation for verifiable claims with a private key that belongs to the given multicipher {@link KeyId}. An
    /// error will be thrown if the JSON does not conform to the schema of a claim presentation.
    ///
    /// The returned {@link SignedJson} has separate properties for signature and public key, but can be serialized into a JSON format
    /// widely used in the IOP Stack™.
    ///
    /// @see keyById
    #[wasm_bindgen(js_name = signClaimPresentation)]
    pub fn sign_claim_presentation(
        &self, id: &JsMKeyId, js_presentation: &JsValue,
    ) -> Result<JsSignedJson, JsValue> {
        let signer = self.create_signer(id)?;
        let presentation: ClaimPresentation = js_presentation.into_serde().map_err(err_to_js)?;
        let signed_presentation =
            signer.sign_claim_presentation(presentation).map_err(err_to_js)?;

        into_signed_json(signed_presentation)
    }

    fn kind_impl(&self, did_kind: DidKind) -> Result<JsMorpheusPrivateKind, JsValue> {
        let inner = self.inner.kind(did_kind).map_err_to_js()?;
        Ok(JsMorpheusPrivateKind::from(inner))
    }

    fn create_signer(&self, id: &JsMKeyId) -> Result<PrivateKeySigner, JsValue> {
        let js_sk = self.key_by_id(id)?;
        let sk: MPrivateKey = js_sk.inner().private_key();
        Ok(PrivateKeySigner::new(sk))
    }
}

impl From<MorpheusPrivate> for JsMorpheusPrivate {
    fn from(inner: MorpheusPrivate) -> Self {
        Self { inner }
    }
}

impl Wraps<MorpheusPrivate> for JsMorpheusPrivate {
    fn inner(&self) -> &MorpheusPrivate {
        &self.inner
    }
}

fn into_signed_json<T: Signable>(signed: Signed<T>) -> Result<JsSignedJson, JsValue> {
    let (public_key, content, signature, nonce) = signed.into_parts();
    let content = serde_json::to_value(content).map_err(err_to_js)?;
    let signed_json = Signed::from_parts(public_key, content, signature, nonce);
    Ok(signed_json.into())
}
