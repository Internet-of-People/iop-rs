use super::*;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Inner {
    parameters: Parameters,
    public_state: PublicState,
}

impl Inner {
    fn new(
        personas: Vec<String>, devices: Vec<String>, groups: Vec<String>, resources: Vec<String>,
    ) -> Self {
        let public_state = PublicState { personas, devices, groups, resources };
        Self { parameters: Default::default(), public_state }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Plugin {
    inner: Arc<RwLock<Inner>>,
}

#[cfg_attr(target_arch = "wasm32", typetag::serialize(name = "Morpheus"))]
#[cfg_attr(not(target_arch = "wasm32"), typetag::serde(name = "Morpheus"))]
impl VaultPlugin for Plugin {
    fn name(&self) -> &'static str {
        "Morpheus"
    }

    fn to_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn eq(&self, other: &dyn VaultPlugin) -> bool {
        let other: Result<Box<Plugin>, _> = other.to_any().downcast();
        other.is_ok()
    }
}

impl Plugin {
    pub fn new(
        personas: Vec<String>, devices: Vec<String>, groups: Vec<String>, resources: Vec<String>,
    ) -> Self {
        let imp = Inner::new(personas, devices, groups, resources);
        let inner = Arc::new(RwLock::new(imp));
        Self { inner }
    }

    pub fn create(vault: &mut Vault) -> Result<()> {
        let plugin = Self::new(vec![], vec![], vec![], vec![]);
        vault.add(Box::new(plugin))
    }

    pub fn init(vault: &mut Vault, unlock_password: impl AsRef<str>) -> Result<()> {
        let seed = vault.unlock(unlock_password.as_ref())?;
        let persona0 = Morpheus.root(&seed)?.personas()?.key(0)?.neuter();
        let plugin = Self::new(vec![persona0.public_key().to_string()], vec![], vec![], vec![]);
        vault.add(Box::new(plugin))
    }

    pub fn get(vault: &Vault) -> Result<BoundPlugin<Plugin, Public, Private>> {
        let morpheus_plugins = vault.plugins_by_type::<Plugin>()?;
        let plugin: &Plugin = morpheus_plugins
            .iter()
            .by_ref()
            .next()
            .with_context(|| "Could not find Morpheus plugin")?;
        Ok(BoundPlugin::new(vault.to_owned(), plugin.to_owned()))
    }

    pub(super) fn to_state(&self) -> Box<dyn State<PublicState>> {
        <dyn State<_>>::map(&self.inner, |s| &s.public_state, |s| &mut s.public_state)
    }
}
