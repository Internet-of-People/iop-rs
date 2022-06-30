use super::*;

/// Represents the Morpheus subtree in a given vault.
#[wasm_bindgen(js_name = MorpheusPlugin)]
pub struct JsMorpheusPlugin {
    inner: MorpheusBoundPlugin,
}

#[wasm_bindgen(js_class = MorpheusPlugin)]
impl JsMorpheusPlugin {
    /// Creates the Morpheus subtree in the vault. If the subtree already exists, an error will be
    /// thrown. An existing subtree has to be retrieved from the vault using {@link get}.
    pub fn init(vault: &mut JsVault, unlock_password: &str) -> Result<(), JsValue> {
        hd_morpheus::Plugin::init(vault.inner_mut(), unlock_password).map_err_to_js()?;
        Ok(())
    }

    /// Retrieves an existing Morpheus subtree from the vault. If the subtree is missing, an error will be thrown. A new subtree can be
    /// created with {@link init}.
    pub fn get(vault: &JsVault) -> Result<JsMorpheusPlugin, JsValue> {
        let inner = hd_morpheus::Plugin::get(vault.inner()).map_err_to_js()?;
        Ok(Self { inner })
    }

    /// Accessor for the public keys in the Morpheus subtree.
    #[wasm_bindgen(getter = pub)]
    pub fn public(&self) -> Result<JsMorpheusPublic, JsValue> {
        let inner = self.inner.public().map_err_to_js()?;
        Ok(JsMorpheusPublic::from(inner))
    }

    /// Accessor for the private keys in the Morpheus subtree. Needs the unlock password.
    ///
    /// @see Vault.unlock
    #[wasm_bindgen(js_name = priv)]
    pub fn private(&self, unlock_password: &str) -> Result<JsMorpheusPrivate, JsValue> {
        let inner = self.inner.private(unlock_password).map_err_to_js()?;
        Ok(JsMorpheusPrivate::from(inner))
    }
}

impl From<MorpheusBoundPlugin> for JsMorpheusPlugin {
    fn from(inner: MorpheusBoundPlugin) -> Self {
        Self { inner }
    }
}

impl Wraps<MorpheusBoundPlugin> for JsMorpheusPlugin {
    fn inner(&self) -> &MorpheusBoundPlugin {
        &self.inner
    }
}
