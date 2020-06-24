use super::*;

#[typetag::serde(tag = "pluginName")]
pub trait VaultPlugin {
    fn name(&self) -> &'static str;
    fn to_any(&self) -> Box<dyn Any>;
    fn eq(&self, other: &dyn VaultPlugin) -> bool;
}

impl fmt::Debug for dyn VaultPlugin {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Vault {
    inner: Rc<RefCell<VaultImpl>>,
}

impl Vault {
    pub fn new(encrypted_seed: String, plugins: Vec<Box<dyn VaultPlugin>>) -> Self {
        let imp = VaultImpl::new(encrypted_seed, plugins);
        let inner = Rc::new(RefCell::new(imp));
        Self { inner }
    }

    pub fn create(
        phrase: impl AsRef<str>, bip39_password: impl AsRef<str>, unlock_password: impl AsRef<str>,
    ) -> Fallible<Vault> {
        let seed = Bip39::new().phrase(phrase)?.password(bip39_password);
        let encrypted_seed = Self::encrypt_seed(&seed, unlock_password.as_ref())?;
        Ok(Self::new(encrypted_seed, Vec::new()))
    }

    pub fn unlock(&self, unlock_password: &str) -> Fallible<Seed> {
        let imp = self.inner.borrow();
        Self::decrypt_seed(&imp.encrypted_seed, unlock_password)
    }

    pub fn plugins_by_type<T: VaultPlugin + 'static>(&self) -> Vec<Box<T>> {
        let imp = self.inner.borrow();
        imp.plugins.iter().by_ref().filter_map(|p| p.to_any().downcast().ok()).collect()
    }

    pub fn add(&mut self, plugin: Box<dyn VaultPlugin>) -> Fallible<()> {
        let mut imp = self.inner.borrow_mut();
        ensure!(
            imp.plugins.iter().all(|p| !p.eq(plugin.as_ref())),
            "Same plugin was already added to vault"
        );
        imp.plugins.push(plugin);
        imp.is_dirty = true;
        Ok(())
    }

    pub fn to_modifiable(&self) -> Box<dyn State<bool>> {
        State::map(&self.inner, |v| &v.is_dirty, |v| &mut v.is_dirty)
    }

    fn encrypt_seed(seed: &Seed, unlock_password: &str) -> Fallible<String> {
        let nonce = nonce()?;
        let encrypted_seed_bytes = encrypt(seed.as_bytes(), unlock_password, nonce)?;
        Ok(multibase::encode(multibase::Base::Base64Url, &encrypted_seed_bytes))
    }

    fn decrypt_seed(seed: &str, unlock_password: &str) -> Fallible<Seed> {
        let (_, encrypted_seed_bytes) = multibase::decode(seed)?;
        let decrypted_bytes = decrypt(&encrypted_seed_bytes, unlock_password)?;
        Seed::from_bytes(&decrypted_bytes)
    }
}

pub trait PluginPublic<T: VaultPlugin>: Sized {
    fn create(plugin: &T, vault_dirty: Box<dyn State<bool>>) -> Fallible<Self>;
}

pub trait PluginPrivate<T: VaultPlugin>: Sized {
    fn create(plugin: &T, seed: Seed, vault_dirty: Box<dyn State<bool>>) -> Fallible<Self>;
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

    pub fn private(&self, unlock_password: impl AsRef<str>) -> Fallible<TPriv> {
        let seed = self.vault.unlock(unlock_password.as_ref())?;
        TPriv::create(&self.plugin, seed, self.vault.to_modifiable())
    }

    pub fn public(&self) -> Fallible<TPublic> {
        TPublic::create(&self.plugin, self.vault.to_modifiable())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const DEMO_VAULT_DAT: &str = r#"
    {
        "encryptedSeed": "uKOE-HCgv-CUHFuL6jCUHMdXrfgGX-nsUM2FwE-5JY0GhSxOFTQSGB4F_N6VwuDYPQ8-q0Q_eQVCpgOsjRzqJAnr8nhyV32yNtpCsGYimpnEjr_enZDOd4jajLjt7b48J7V5yDKKVyp8",
        "plugins": [
            {
                "pluginName": "Hydra",
                "parameters": {
                    "network": "HYD testnet",
                    "account": 0
                },
                "publicState": {
                    "xpub": "hydtVxG6GvapCX2X1YxnwKWGzh8tKy6X56gQUN2KRVpqXkgZQYDE7jNw24ZK23ZXEow4cfJz41fBpRj1wV5mbLBYfdpcRgZuS4mSZ22LsVugPZFK",
                    "receiveKeys": 2,
                    "changeKeys": 0
                }
            },
            {
                "pluginName": "Morpheus",
                "parameters": {},
                "publicState": {
                    "personas": [
                        "pez2CLkBUjHB8w8G87D3YkREjpRuiqPu6BrRsgHMQy2Pzt6",
                        "pezDj6ea4tVfNRUTMyssVDepAAzPW67Fe3yHtuHL6ZNtcfJ",
                        "pezsfLDb1fngso3J7TXU6jP3nSr2iubcJZ4KXanxrhs9gr"
                    ]
                }
            }
        ]
    }"#;

    #[test]
    fn serialize() -> Fallible<()> {
        let unlock_password = "correct horse battery staple";
        let vault: Vault = serde_json::from_str(DEMO_VAULT_DAT)?;

        let hyd = hydra::Plugin::get(&vault, &hydra::Parameters::new(&hyd::Testnet, 0))?;

        let hyd_private = hyd.private(unlock_password)?;
        let hyd_pk: SecpPublicKey =
            "02db11c07afd6ec05980284af58105329d41e9882947188022350219cca9baa3e7".parse()?;
        let hyd0 = hyd_private.key_by_pk(&hyd_pk)?;
        let addr = hyd0.neuter().to_p2pkh_addr();

        assert_eq!(&addr, "tjMvaU79mMJ8fKwoLjFLn7rCTthpY6KxTx");

        let m = morpheus::Plugin::get(&vault)?;

        let m_private = m.private(unlock_password)?;
        let m_pk: MPublicKey = "pez2CLkBUjHB8w8G87D3YkREjpRuiqPu6BrRsgHMQy2Pzt6".parse()?;
        let persona0 = m_private.key_by_pk(&m_pk)?;
        let did = Did::from(persona0.neuter().public_key().key_id());

        assert_eq!(&did.to_string(), "did:morpheus:ezqztJ6XX6GDxdSgdiySiT3J");

        Ok(())
    }
}
