use super::*;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Inner {
    parameters: Parameters,
    public_state: PublicState,
}

impl Inner {
    fn new(parameters: Parameters, xpub: String, receive_keys: u32, change_keys: u32) -> Self {
        let public_state = PublicState { xpub, receive_keys, change_keys };
        Self { parameters, public_state }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Plugin {
    inner: Rc<RefCell<Inner>>,
}

#[typetag::serde(name = "Hydra")]
impl VaultPlugin for Plugin {
    fn name(&self) -> &'static str {
        "Hydra"
    }

    fn to_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn eq(&self, other: &dyn VaultPlugin) -> bool {
        let other: Result<Box<Plugin>, _> = other.to_any().downcast();
        match other {
            Ok(p) => {
                let lhs = self.inner.borrow();
                let rhs = p.inner.borrow();
                lhs.parameters == rhs.parameters
            }
            Err(_) => false,
        }
    }
}

impl Plugin {
    pub fn new(parameters: Parameters, xpub: String, receive_keys: u32, change_keys: u32) -> Self {
        let imp = Inner::new(parameters, xpub, receive_keys, change_keys);
        let inner = Rc::new(RefCell::new(imp));
        Self { inner }
    }

    pub fn rewind(
        vault: &mut Vault, unlock_password: impl AsRef<str>, parameters: &Parameters,
    ) -> Fallible<()> {
        let seed = vault.unlock(unlock_password.as_ref())?;
        let account = Self::create_account(parameters, &seed)?;
        let pk: Bip44PublicAccount<Secp256k1> = account.neuter();
        let plugin = Self::new(parameters.to_owned(), pk.to_xpub(), 1, 0);
        vault.add(Box::new(plugin))?;
        Ok(())
    }

    pub fn get(
        vault: &Vault, parameters: &Parameters,
    ) -> Fallible<BoundPlugin<Plugin, Public, Private>> {
        let _network = Networks::by_name(&parameters.network)?; // checks if network name is supported
        ensure!(parameters.account >= 0, "Hydra account number cannot be negative");

        let hydra_plugins = vault.plugins_by_type::<Plugin>();
        let plugin: &Plugin = hydra_plugins
            .iter()
            .by_ref()
            .find(|p| p.inner.borrow().parameters == *parameters)
            .ok_or_else(|| err_msg("Could not find Hydra plugin with given parameters"))?;
        Ok(BoundPlugin::new(vault.to_owned(), plugin.to_owned()))
    }

    pub fn xpub(&self) -> String {
        let imp = self.inner.borrow();
        imp.public_state.xpub.to_owned()
    }

    pub fn receive_keys(&self) -> u32 {
        let imp = self.inner.borrow();
        imp.public_state.receive_keys
    }

    pub fn change_keys(&self) -> u32 {
        let imp = self.inner.borrow();
        imp.public_state.change_keys
    }

    pub fn network(&self) -> &'static dyn Network<Suite = Secp256k1> {
        let imp = self.inner.borrow();
        Networks::by_name(&imp.parameters.network).unwrap()
    }

    pub fn account(&self) -> i32 {
        let imp = self.inner.borrow();
        imp.parameters.account
    }

    pub(super) fn touch_receive_idx(
        &mut self, idx: i32, vault_dirty: &mut dyn State<bool>,
    ) -> Fallible<()> {
        ensure!(idx >= 0, "Key index cannot be negative");
        let required_keys = (idx as u32) + 1;
        let receive_keys = {
            let imp = self.inner.borrow();
            imp.public_state.receive_keys
        };
        if required_keys > receive_keys {
            let mut imp = self.inner.borrow_mut();
            imp.public_state.receive_keys = required_keys;
            let mut dirty = vault_dirty.try_borrow_mut()?;
            *dirty = true;
        }
        Ok(())
    }

    fn create_account(parameters: &Parameters, seed: &Seed) -> Fallible<Bip44Account<Secp256k1>> {
        let network = Networks::by_name(&parameters.network)?;
        Bip44.network(seed, network)?.account(parameters.account)
    }
}
