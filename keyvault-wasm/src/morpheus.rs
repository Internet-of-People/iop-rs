use super::*;

/// Starting point for deriving all Morpheus related keys in a BIP32 hierarchy. Morpheus uses Ed25519 cipher and currently there are no
/// WASM wrappers for Bip32 nodes with that cipher. Still, Bip32 paths are returned by each object so compatible wallets can derive the
/// same extended private keys.
#[wasm_bindgen(js_name = Morpheus)]
#[derive(Clone, Debug)]
pub struct JsMorpheus;

#[wasm_bindgen(js_class = Morpheus)]
impl JsMorpheus {
    /// Calculate the root node of the Morpheus subtree in the HD wallet defined by a seed.
    pub fn root(seed: &JsSeed) -> Result<JsMorpheusRoot, JsValue> {
        let inner = Morpheus.root(seed.inner()).map_err_to_js()?;
        Ok(JsMorpheusRoot::from(inner))
    }
}

/// Representation of the root node of the Morpheus subtree in the HD wallet.
#[wasm_bindgen(js_name = MorpheusRoot)]
#[derive(Clone, Debug)]
pub struct JsMorpheusRoot {
    inner: MorpheusRoot,
}

#[wasm_bindgen(js_class = MorpheusRoot)]
impl JsMorpheusRoot {
    /// Accessor for the BIP32 path of the morpheus root.
    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.node().path().to_string()
    }

    /// Derive a separate HD wallet subtree of the given DID kind. Use 'persona', 'device', 'group' or 'resource' in
    /// singular as a parameter.
    pub fn kind(&self, did_kind: &str) -> Result<JsMorpheusKind, JsValue> {
        let did_kind = did_kind.parse().map_err_to_js()?;
        self.kind_impl(did_kind)
    }

    /// Alias for kind('persona')
    pub fn personas(&self) -> Result<JsMorpheusKind, JsValue> {
        self.kind_impl(DidKind::Persona)
    }

    /// Alias for kind('device')
    pub fn devices(&self) -> Result<JsMorpheusKind, JsValue> {
        self.kind_impl(DidKind::Device)
    }

    /// Alias for kind('group')
    pub fn groups(&self) -> Result<JsMorpheusKind, JsValue> {
        self.kind_impl(DidKind::Group)
    }

    /// Alias for kind('resource')
    pub fn resources(&self) -> Result<JsMorpheusKind, JsValue> {
        self.kind_impl(DidKind::Resource)
    }

    fn kind_impl(&self, did_kind: DidKind) -> Result<JsMorpheusKind, JsValue> {
        let inner = self.inner.kind(did_kind).map_err_to_js()?;
        Ok(JsMorpheusKind::from(inner))
    }
}

impl From<MorpheusRoot> for JsMorpheusRoot {
    fn from(inner: MorpheusRoot) -> Self {
        Self { inner }
    }
}

impl Wraps<MorpheusRoot> for JsMorpheusRoot {
    fn inner(&self) -> &MorpheusRoot {
        &self.inner
    }
}

/// Root node of a specific kind of DIDs. The kind used to derive a DID is indistiguishable outside the wallet.
#[wasm_bindgen(js_name = MorpheusKind)]
#[derive(Clone, Debug)]
pub struct JsMorpheusKind {
    inner: MorpheusKind,
}

#[wasm_bindgen(js_class = MorpheusKind)]
impl JsMorpheusKind {
    /// Accessor for the BIP32 path of the morpheus subtree for a DID kind.
    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.node().path().to_string()
    }

    /// Accessor for the kind of DIDs in this subtree
    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        format!("{:?}", self.inner.path())
    }

    /// Creates a {@link MorpheusPrivateKey} with the given index under this subtree.
    /// E.g. 5th persona, 3rd device, or 0th group, etc.
    pub fn key(&self, idx: i32) -> Result<JsMorpheusPrivateKey, JsValue> {
        let inner = self.inner.key(idx).map_err_to_js()?;
        Ok(JsMorpheusPrivateKey::from(inner))
    }
}

impl From<MorpheusKind> for JsMorpheusKind {
    fn from(inner: MorpheusKind) -> Self {
        Self { inner }
    }
}

impl Wraps<MorpheusKind> for JsMorpheusKind {
    fn inner(&self) -> &MorpheusKind {
        &self.inner
    }
}

/// The operations on an identifier that require the private key to be available in memory.
#[wasm_bindgen(js_name = MorpheusPrivateKey)]
#[derive(Clone, Debug)]
pub struct JsMorpheusPrivateKey {
    inner: MorpheusPrivateKey,
}

#[wasm_bindgen(js_class = MorpheusPrivateKey)]
impl JsMorpheusPrivateKey {
    /// Accessor for the BIP32 path of the morpheus key.
    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.node().path().to_string()
    }

    /// Accessor for the kind of DIDs in this subtree
    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        format!("{:?}", self.inner.path().kind())
    }

    /// Index of the key in its subtree.
    #[wasm_bindgen(getter)]
    pub fn idx(&self) -> i32 {
        self.inner().path().idx()
    }

    /// Creates the public interface of the node that does not need the private key in memory.
    pub fn neuter(&self) -> JsMorpheusPublicKey {
        let inner = self.inner.neuter();
        JsMorpheusPublicKey::from(inner)
    }

    /// Returns the multicipher {@link PrivateKey} that belongs to this key.
    #[wasm_bindgen(js_name = privateKey)]
    pub fn private_key(&self) -> JsMPrivateKey {
        let inner = self.inner.private_key();
        JsMPrivateKey::from(inner)
    }
}

impl From<MorpheusPrivateKey> for JsMorpheusPrivateKey {
    fn from(inner: MorpheusPrivateKey) -> Self {
        Self { inner }
    }
}

impl Wraps<MorpheusPrivateKey> for JsMorpheusPrivateKey {
    fn inner(&self) -> &MorpheusPrivateKey {
        &self.inner
    }
}

/// The operations on an identifier that do not require the private key to be available in memory.
#[wasm_bindgen(js_name = MorpheusPublicKey)]
#[derive(Clone, Debug)]
pub struct JsMorpheusPublicKey {
    inner: MorpheusPublicKey,
}

#[wasm_bindgen(js_class = MorpheusPublicKey)]
impl JsMorpheusPublicKey {
    /// Accessor for the BIP32 path of the morpheus key.
    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.node().path().to_string()
    }

    /// Accessor for the kind of DIDs in this subtree
    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        format!("{:?}", self.inner.path().kind())
    }

    /// Index of the key in its subtree.
    #[wasm_bindgen(getter)]
    pub fn idx(&self) -> i32 {
        self.inner().path().idx()
    }

    /// Returns the multicipher {@link PublicKey} that belongs to this key.
    #[wasm_bindgen(js_name = publicKey)]
    pub fn public_key(&self) -> JsMPublicKey {
        let inner = self.inner.public_key();
        JsMPublicKey::from(inner)
    }
}

impl From<MorpheusPublicKey> for JsMorpheusPublicKey {
    fn from(inner: MorpheusPublicKey) -> Self {
        Self { inner }
    }
}

impl Wraps<MorpheusPublicKey> for JsMorpheusPublicKey {
    fn inner(&self) -> &MorpheusPublicKey {
        &self.inner
    }
}
