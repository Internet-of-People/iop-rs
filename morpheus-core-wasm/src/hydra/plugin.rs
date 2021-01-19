use super::*;

#[wasm_bindgen(js_name = HydraPlugin)]
pub struct JsHydraPlugin {
    inner: HydraBoundPlugin,
}

#[wasm_bindgen(js_class = HydraPlugin)]
impl JsHydraPlugin {
    pub fn init(
        vault: &mut JsVault, unlock_password: &str, parameters: &JsHydraParameters,
    ) -> Result<(), JsValue> {
        hd_hydra::Plugin::init(vault.inner_mut(), unlock_password, parameters.inner())
            .map_err_to_js()?;
        Ok(())
    }

    pub fn get(vault: &JsVault, parameters: &JsHydraParameters) -> Result<JsHydraPlugin, JsValue> {
        let inner = hd_hydra::Plugin::get(vault.inner(), parameters.inner()).map_err_to_js()?;
        Ok(Self { inner })
    }

    #[wasm_bindgen(getter = pub)]
    pub fn public(&self) -> Result<JsHydraPublic, JsValue> {
        let inner = self.inner.public().map_err_to_js()?;
        Ok(JsHydraPublic::from(inner))
    }

    #[wasm_bindgen(js_name = priv)]
    pub fn private(&self, unlock_password: &str) -> Result<JsHydraPrivate, JsValue> {
        let inner = self.inner.private(unlock_password).map_err_to_js()?;
        Ok(JsHydraPrivate::from(inner))
    }
}

impl From<HydraBoundPlugin> for JsHydraPlugin {
    fn from(inner: HydraBoundPlugin) -> Self {
        Self { inner }
    }
}

impl Wraps<HydraBoundPlugin> for JsHydraPlugin {
    fn inner(&self) -> &HydraBoundPlugin {
        &self.inner
    }
}
