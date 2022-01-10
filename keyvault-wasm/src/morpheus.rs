use super::*;

#[wasm_bindgen(js_name = Morpheus)]
#[derive(Clone, Debug)]
pub struct JsMorpheus;

#[wasm_bindgen(js_class = Morpheus)]
impl JsMorpheus {
    pub fn root(seed: &JsSeed) -> Result<JsMorpheusRoot, JsValue> {
        let inner = Morpheus.root(seed.inner()).map_err_to_js()?;
        Ok(JsMorpheusRoot::from(inner))
    }
}

#[wasm_bindgen(js_name = MorpheusRoot)]
#[derive(Clone, Debug)]
pub struct JsMorpheusRoot {
    inner: MorpheusRoot,
}

#[wasm_bindgen(js_class = MorpheusRoot)]
impl JsMorpheusRoot {
    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.node().path().to_string()
    }

    pub fn kind(&self, did_kind: &str) -> Result<JsMorpheusKind, JsValue> {
        let did_kind = did_kind.parse().map_err_to_js()?;
        self.kind_impl(did_kind)
    }

    pub fn personas(&self) -> Result<JsMorpheusKind, JsValue> {
        self.kind_impl(DidKind::Persona)
    }

    pub fn devices(&self) -> Result<JsMorpheusKind, JsValue> {
        self.kind_impl(DidKind::Device)
    }

    pub fn groups(&self) -> Result<JsMorpheusKind, JsValue> {
        self.kind_impl(DidKind::Group)
    }

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

#[wasm_bindgen(js_name = MorpheusKind)]
#[derive(Clone, Debug)]
pub struct JsMorpheusKind {
    inner: MorpheusKind,
}

#[wasm_bindgen(js_class = MorpheusKind)]
impl JsMorpheusKind {
    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.node().path().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        format!("{:?}", self.inner.path())
    }

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

#[wasm_bindgen(js_name = MorpheusPrivateKey)]
#[derive(Clone, Debug)]
pub struct JsMorpheusPrivateKey {
    inner: MorpheusPrivateKey,
}

#[wasm_bindgen(js_class = MorpheusPrivateKey)]
impl JsMorpheusPrivateKey {
    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.node().path().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        format!("{:?}", self.inner.path().kind())
    }

    #[wasm_bindgen(getter)]
    pub fn idx(&self) -> i32 {
        self.inner().path().idx()
    }

    pub fn neuter(&self) -> JsMorpheusPublicKey {
        let inner = self.inner.neuter();
        JsMorpheusPublicKey::from(inner)
    }

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

#[wasm_bindgen(js_name = MorpheusPublicKey)]
#[derive(Clone, Debug)]
pub struct JsMorpheusPublicKey {
    inner: MorpheusPublicKey,
}

#[wasm_bindgen(js_class = MorpheusPublicKey)]
impl JsMorpheusPublicKey {
    #[wasm_bindgen(getter = path)]
    pub fn bip32_path(&self) -> String {
        self.inner.node().path().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> String {
        format!("{:?}", self.inner.path().kind())
    }

    #[wasm_bindgen(getter)]
    pub fn idx(&self) -> i32 {
        self.inner().path().idx()
    }

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
