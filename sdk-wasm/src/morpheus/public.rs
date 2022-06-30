use super::*;

/// Public keys of the Morpheus subtree in a vault.
///
/// @see MorpheusPlugin.priv
#[wasm_bindgen(js_name = MorpheusPublic)]
pub struct JsMorpheusPublic {
    inner: MorpheusPublic,
}

#[wasm_bindgen(js_class = MorpheusPublic)]
impl JsMorpheusPublic {
    /// There can be several usages of DIDs differentiated inside the vault invisible externally, e.g. on a blockchain.
    /// Each represents a separate subtree under the Morpheus subtree in the vault.
    ///
    /// Use 'persona', 'device', 'group' or 'resource' in singular as a parameter.
    pub fn kind(&self, did_kind: &str) -> Result<JsMorpheusPublicKind, JsValue> {
        let did_kind: DidKind = did_kind.parse().map_err_to_js()?;
        self.kind_impl(did_kind)
    }

    /// Alias for {@link kind('persona')}
    #[wasm_bindgen(getter)]
    pub fn personas(&self) -> Result<JsMorpheusPublicKind, JsValue> {
        self.kind_impl(DidKind::Persona)
    }

    /// Alias for {@link kind('device')}
    #[wasm_bindgen(getter)]
    pub fn devices(&self) -> Result<JsMorpheusPublicKind, JsValue> {
        self.kind_impl(DidKind::Device)
    }

    /// Alias for {@link kind('group')}
    #[wasm_bindgen(getter)]
    pub fn groups(&self) -> Result<JsMorpheusPublicKind, JsValue> {
        self.kind_impl(DidKind::Group)
    }

    /// Alias for {@link kind('resource')}
    #[wasm_bindgen(getter)]
    pub fn resources(&self) -> Result<JsMorpheusPublicKind, JsValue> {
        self.kind_impl(DidKind::Resource)
    }

    /// Finds the multicipher {@link PublicKey} that belongs to the given multicipher {@link KeyId}.
    ///
    /// An error will be thrown if the key identifier has never been used yet in this vault.
    #[wasm_bindgen(js_name = keyById)]
    pub fn key_by_id(&self, id: &JsMKeyId) -> Result<JsMPublicKey, JsValue> {
        let inner = self.inner.key_by_id(id.inner()).map_err_to_js()?;
        Ok(JsMPublicKey::from(inner))
    }

    fn kind_impl(&self, did_kind: DidKind) -> Result<JsMorpheusPublicKind, JsValue> {
        let inner = self.inner.kind(did_kind).map_err_to_js()?;
        Ok(JsMorpheusPublicKind::from(inner))
    }
}

impl From<MorpheusPublic> for JsMorpheusPublic {
    fn from(inner: MorpheusPublic) -> Self {
        Self { inner }
    }
}

impl Wraps<MorpheusPublic> for JsMorpheusPublic {
    fn inner(&self) -> &MorpheusPublic {
        &self.inner
    }
}
