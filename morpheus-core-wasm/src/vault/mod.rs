mod serializer;

use serializer::VaultSerializer;

use super::*;

#[wasm_bindgen(js_name = Vault)]
pub struct JsVault {
    inner: Vault,
}

#[wasm_bindgen(js_class = Vault)]
impl JsVault {
    pub fn create(
        phrase: &str, bip39_password: &str, unlock_password: &str, language: Option<String>,
    ) -> Result<JsVault, JsValue> {
        let inner = Vault::create(language.as_deref(), phrase, bip39_password, unlock_password)
            .map_err_to_js()?;
        Ok(Self { inner })
    }

    pub fn load(data: &JsValue) -> Result<JsVault, JsValue> {
        // TODO Consider https://github.com/cloudflare/serde-wasm-bindgen
        let data_serde: serde_json::Value = data.into_serde().map_err_to_js()?;
        let hack: VaultSerializer = serde_json::from_value(data_serde).map_err_to_js()?;
        let inner = Vault::from(hack);
        Ok(Self { inner })
    }

    pub fn save(&mut self) -> Result<JsValue, JsValue> {
        let result = JsValue::from_serde(&self.inner).map_err_to_js()?;
        self.set_dirty(false)?;
        Ok(result)
    }

    #[wasm_bindgen(getter = dirty)]
    pub fn is_dirty(&self) -> Result<bool, JsValue> {
        let vault_dirty = self.inner.to_modifiable();
        let dirty = vault_dirty.try_borrow().map_err_to_js()?;
        Ok(*dirty)
    }

    #[wasm_bindgen(js_name = setDirty)]
    pub fn set_to_dirty(&mut self) -> Result<(), JsValue> {
        self.set_dirty(true)?;
        Ok(())
    }

    pub fn unlock(&self, password: &str) -> Result<JsSeed, JsValue> {
        let seed = self.inner.unlock(password).map_err_to_js()?;
        Ok(JsSeed::from(seed))
    }

    fn set_dirty(&mut self, value: bool) -> Result<(), JsValue> {
        let mut vault_dirty = self.inner.to_modifiable();
        let mut dirty = vault_dirty.try_borrow_mut().map_err_to_js()?;
        *dirty = value;
        Ok(())
    }
}

impl From<Vault> for JsVault {
    fn from(inner: Vault) -> Self {
        Self { inner }
    }
}

impl Wraps<Vault> for JsVault {
    fn inner(&self) -> &Vault {
        &self.inner
    }
}

impl WrapsMut<Vault> for JsVault {
    fn inner_mut(&mut self) -> &mut Vault {
        &mut self.inner
    }
}
