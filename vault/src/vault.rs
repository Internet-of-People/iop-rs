use super::*;

#[cfg_attr(target_arch = "wasm32", typetag::serialize(tag = "pluginName"))]
#[cfg_attr(not(target_arch = "wasm32"), typetag::serde(tag = "pluginName"))]
pub trait VaultPlugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn to_any(&self) -> Box<dyn Any>;
    fn eq(&self, other: &dyn VaultPlugin) -> bool;
}

impl fmt::Debug for dyn VaultPlugin {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

#[cfg_attr(not(target_arch = "wasm32"), derive(Deserialize))]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct VaultImpl {
    encrypted_seed: String,
    plugins: Vec<Box<dyn VaultPlugin>>,
    #[serde(skip)]
    is_dirty: bool, // Plugins and language bindings are trusted to use this properly
}

impl VaultImpl {
    fn new(encrypted_seed: String, plugins: Vec<Box<dyn VaultPlugin>>) -> Self {
        let is_dirty = false;
        Self { encrypted_seed, plugins, is_dirty }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), derive(Deserialize))]
#[derive(Clone, Debug, Serialize)]
#[serde(transparent)]
pub struct Vault {
    inner: Arc<RwLock<VaultImpl>>,
}

impl Vault {
    pub fn new(encrypted_seed: String, plugins: Vec<Box<dyn VaultPlugin>>) -> Self {
        let imp = VaultImpl::new(encrypted_seed, plugins);
        let inner = Arc::new(RwLock::new(imp));
        Self { inner }
    }

    pub fn create(
        lang_code: Option<&str>, phrase: impl AsRef<str>, bip39_password: impl AsRef<str>,
        unlock_password: impl AsRef<str>,
    ) -> Result<Vault> {
        let bip39 = match lang_code {
            None => Bip39::new(),
            Some(code) => Bip39::language_code(code)?,
        };
        let seed = bip39.phrase(phrase)?.password(bip39_password);
        let encrypted_seed = Self::encrypt_seed(&seed, unlock_password.as_ref())?;
        Ok(Self::new(encrypted_seed, Vec::new()))
    }

    pub fn unlock(&self, unlock_password: &str) -> Result<Seed> {
        let imp = self.inner.try_read().ok_or_else(|| format_err!("Read lock on Vault failed"))?;
        Self::decrypt_seed(&imp.encrypted_seed, unlock_password)
    }

    pub fn plugins_by_type<T: VaultPlugin + 'static>(&self) -> Result<Vec<Box<T>>> {
        let imp = self.inner.try_read().ok_or_else(|| format_err!("Read lock on Vault failed"))?;
        let plugins =
            imp.plugins.iter().by_ref().filter_map(|p| p.to_any().downcast().ok()).collect();
        Ok(plugins)
    }

    pub fn add(&mut self, plugin: Box<dyn VaultPlugin>) -> Result<()> {
        let mut imp =
            self.inner.try_write().ok_or_else(|| format_err!("Write lock on Vault failed"))?;
        ensure!(
            imp.plugins.iter().all(|p| !p.eq(plugin.as_ref())),
            "Same plugin was already added to vault"
        );
        imp.plugins.push(plugin);
        imp.is_dirty = true;
        Ok(())
    }

    pub fn to_modifiable(&self) -> Box<dyn State<bool>> {
        <dyn State<_>>::map(&self.inner, |v| &v.is_dirty, |v| &mut v.is_dirty)
    }

    fn encrypt_seed(seed: &Seed, unlock_password: &str) -> Result<String> {
        let nonce = nonce()?;
        let encrypted_seed_bytes = encrypt(seed.as_bytes(), unlock_password, nonce)?;
        Ok(multibase::encode(multibase::Base::Base64Url, &encrypted_seed_bytes))
    }

    fn decrypt_seed(seed: &str, unlock_password: &str) -> Result<Seed> {
        let (_, encrypted_seed_bytes) = multibase::decode(seed)?;
        let decrypted_bytes = decrypt(&encrypted_seed_bytes, unlock_password)?;
        Seed::from_bytes(&decrypted_bytes)
    }
}

pub trait PluginPublic<T: VaultPlugin>: Sized {
    fn create(plugin: &T, vault_dirty: Box<dyn State<bool>>) -> Result<Self>;
}

pub trait PluginPrivate<T: VaultPlugin>: Sized {
    fn create(plugin: &T, seed: Seed, vault_dirty: Box<dyn State<bool>>) -> Result<Self>;
}

pub struct BoundPlugin<T: VaultPlugin, TPublic: PluginPublic<T>, TPriv: PluginPrivate<T>> {
    vault: Vault,
    plugin: T,
    _pub: PhantomData<TPublic>,
    _priv: PhantomData<TPriv>,
}

impl<T: VaultPlugin, TPublic: PluginPublic<T>, TPriv: PluginPrivate<T>>
    BoundPlugin<T, TPublic, TPriv>
{
    pub fn new(vault: Vault, plugin: T) -> Self {
        Self { vault, plugin, _pub: Default::default(), _priv: Default::default() }
    }

    pub fn private(&self, unlock_password: impl AsRef<str>) -> Result<TPriv> {
        let seed = self.vault.unlock(unlock_password.as_ref())?;
        TPriv::create(&self.plugin, seed, self.vault.to_modifiable())
    }

    pub fn public(&self) -> Result<TPublic> {
        TPublic::create(&self.plugin, self.vault.to_modifiable())
    }
}
