use super::*;

#[wasm_bindgen(js_name = MorpheusPlugin)]
pub struct JsMorpheusPlugin {
    inner: MorpheusBoundPlugin,
}

#[wasm_bindgen(js_class = MorpheusPlugin)]
impl JsMorpheusPlugin {
    pub fn init(vault: &mut JsVault, unlock_password: &str) -> Result<(), JsValue> {
        hd_morpheus::Plugin::init(vault.inner_mut(), unlock_password).map_err_to_js()?;
        Ok(())
    }

    pub fn get(vault: &JsVault) -> Result<JsMorpheusPlugin, JsValue> {
        let inner = hd_morpheus::Plugin::get(vault.inner()).map_err_to_js()?;
        Ok(Self { inner })
    }

    #[wasm_bindgen(getter = pub)]
    pub fn public(&self) -> Result<JsMorpheusPublic, JsValue> {
        let inner = self.inner.public().map_err_to_js()?;
        Ok(JsMorpheusPublic::from(inner))
    }

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
