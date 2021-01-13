use super::*;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Inner {
    parameters: Parameters,
    public_state: PublicState,
}

impl Inner {
    fn new(personas: Vec<String>) -> Self {
        let public_state = PublicState { personas };
        Self { parameters: Default::default(), public_state }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(from = "Inner")]
pub struct Plugin {
    inner: Arc<RwLock<Inner>>,
}

impl From<Inner> for Plugin {
    fn from(inner: Inner) -> Self {
        Self { inner: Arc::new(RwLock::new(inner)) }
    }
}

impl Serialize for Plugin {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let r = self.inner.try_borrow().map_err(|e| {
            SerializerError::custom(format!("Failed to lock vault for serialization: {}", e))
        })?;
        (*r).serialize(s)
    }
}

#[typetag::serde(name = "Morpheus")]
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
    pub fn new(personas: Vec<String>) -> Self {
        let imp = Inner::new(personas);
        let inner = Arc::new(RwLock::new(imp));
        Self { inner }
    }

    pub fn init(vault: &mut Vault, unlock_password: impl AsRef<str>) -> Result<()> {
        let seed = vault.unlock(unlock_password.as_ref())?;
        let persona0 = Morpheus.root(&seed)?.personas()?.key(0)?.neuter();
        let plugin = Self::new(vec![persona0.public_key().to_string()]);
        vault.add(Box::new(plugin))?;
        Ok(())
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
        State::map(&self.inner, |s| &s.public_state, |s| &mut s.public_state)
    }
}
