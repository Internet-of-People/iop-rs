use super::*;

#[wasm_bindgen(js_name = MorpheusPrivate)]
pub struct JsMorpheusPrivate {
    inner: MorpheusPrivate,
}

#[wasm_bindgen(js_class = MorpheusPrivate)]
impl JsMorpheusPrivate {
    #[wasm_bindgen(getter = pub)]
    pub fn public(&self) -> JsMorpheusPublic {
        let inner = self.inner.public();
        JsMorpheusPublic::from(inner)
    }

    pub fn kind(&self, did_kind: &str) -> Result<JsMorpheusPrivateKind, JsValue> {
        let did_kind: DidKind = did_kind.parse().map_err_to_js()?;
        self.kind_impl(did_kind)
    }

    #[wasm_bindgen(getter)]
    pub fn personas(&self) -> Result<JsMorpheusPrivateKind, JsValue> {
        self.kind_impl(DidKind::Persona)
    }

    #[wasm_bindgen(getter)]
    pub fn devices(&self) -> Result<JsMorpheusPrivateKind, JsValue> {
        self.kind_impl(DidKind::Device)
    }

    #[wasm_bindgen(getter)]
    pub fn groups(&self) -> Result<JsMorpheusPrivateKind, JsValue> {
        self.kind_impl(DidKind::Group)
    }

    #[wasm_bindgen(getter)]
    pub fn resources(&self) -> Result<JsMorpheusPrivateKind, JsValue> {
        self.kind_impl(DidKind::Resource)
    }

    #[wasm_bindgen(js_name = keyByPublicKey)]
    pub fn key_by_pk(&self, pk: &JsMPublicKey) -> Result<JsMorpheusPrivateKey, JsValue> {
        let inner = self.inner.key_by_pk(pk.inner()).map_err_to_js()?;
        Ok(JsMorpheusPrivateKey::from(inner))
    }

    #[wasm_bindgen(js_name = keyById)]
    pub fn key_by_id(&self, id: &JsMKeyId) -> Result<JsMorpheusPrivateKey, JsValue> {
        let pk = self.inner.public().key_by_id(id.inner()).map_err_to_js()?;
        let js_pk = JsMPublicKey::from(pk);
        self.key_by_pk(&js_pk)
    }

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

    #[wasm_bindgen(js_name = signWitnessRequest)]
    pub fn sign_witness_request(
        &self, id: &JsMKeyId, js_req: &JsValue,
    ) -> Result<JsSignedJson, JsValue> {
        let signer = self.create_signer(id)?;
        let request: WitnessRequest = js_req.into_serde().map_err(err_to_js)?;
        let signed_request = signer.sign_witness_request(request).map_err(err_to_js)?;

        into_signed_json(signed_request)
    }

    #[wasm_bindgen(js_name = signWitnessStatement)]
    pub fn sign_witness_statement(
        &self, id: &JsMKeyId, js_stmt: &JsValue,
    ) -> Result<JsSignedJson, JsValue> {
        let signer = self.create_signer(id)?;
        let statement: WitnessStatement = js_stmt.into_serde().map_err(err_to_js)?;
        let signed_statement = signer.sign_witness_statement(statement).map_err(err_to_js)?;

        into_signed_json(signed_statement)
    }

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
