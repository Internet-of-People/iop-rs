mod serializer;

use serializer::VaultSerializer;

use super::*;

/// This object provides a safe serialization format for an in-rest encoded vault file for the IOP Stack™.
#[wasm_bindgen(js_name = Vault)]
pub struct JsVault {
    inner: Vault,
}

#[wasm_bindgen(js_class = Vault)]
impl JsVault {
    /// Creates a new in-memory vault object from a BIP39 phrase, a seed password (aka 25th word), an unlock password used for
    /// encryption of the secrets in rest, and optionally a language code (e.g. 'zh-hans' or 'es') for the BIP39 phrase words ('en' by
    /// default).
    pub fn create(
        phrase: &str, bip39_password: &str, unlock_password: &str, language: Option<String>,
    ) -> Result<JsVault, JsValue> {
        let inner = Vault::create(language.as_deref(), phrase, bip39_password, unlock_password)
            .map_err_to_js()?;
        Ok(Self { inner })
    }

    /// Loads the vault from its JSON serialization format. Note that no private keys can be calculated without unlocking the loaded
    /// vault with {@link unlock} or with some plugins like {@link HydraPlugin.private} or {@link MorpheusPlugin.private}. The public
    /// keys can be enumerated and used without the unlock password.
    pub fn load(data: &JsValue) -> Result<JsVault, JsValue> {
        // TODO Consider https://github.com/cloudflare/serde-wasm-bindgen
        let data_serde: serde_json::Value = data.into_serde().map_err_to_js()?;
        let hack: VaultSerializer = serde_json::from_value(data_serde).map_err_to_js()?;
        let inner = Vault::from(hack);
        Ok(Self { inner })
    }

    /// Saves the vault into its JSON serialization format. The private keys are encrypted with the unlock password, but the public
    /// keys can be enumerated from the file, so make sure you understand the privacy aspects of sharing such file with 3rd parties.
    ///
    /// Note that calling this method clears the {@link dirty} flag on the vault.
    pub fn save(&mut self) -> Result<JsValue, JsValue> {
        let result = JsValue::from_serde(&self.inner).map_err_to_js()?;
        self.set_dirty(false)?;
        Ok(result)
    }

    /// Returns whether the vault has changes since it has been last saved.
    ///
    /// @see save
    #[wasm_bindgen(getter = dirty)]
    pub fn is_dirty(&self) -> Result<bool, JsValue> {
        let vault_dirty = self.inner.to_modifiable();
        let dirty = vault_dirty.try_borrow().map_err_to_js()?;
        Ok(*dirty)
    }

    /// Manually sets the dirty flag on the vault.
    #[wasm_bindgen(js_name = setDirty)]
    pub fn set_to_dirty(&mut self) -> Result<(), JsValue> {
        self.set_dirty(true)?;
        Ok(())
    }

    /// Unlocks the secrets in the vault with a password. Make sure the password is difficult to guess. Good passwords are a few words
    /// randomly picked from huge dictionaries, like what the passphrase option of the [Bitwarden password
    /// generator](https://bitwarden.com/password-generator/) creates (See [correct horse battery staple](https://xkcd.com/936/)).
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
