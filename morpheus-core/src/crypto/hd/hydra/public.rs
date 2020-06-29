use super::*;

pub struct Public {
    state: Box<dyn State<PublicState>>,
    account: Bip44PublicAccount<Secp256k1>,
    vault_dirty: Box<dyn State<bool>>,
}

impl PluginPublic<Plugin> for Public {
    fn create(plugin: &Plugin, vault_dirty: Box<dyn State<bool>>) -> Fallible<Self> {
        let network = plugin.network();
        let account = plugin.account();
        let state = plugin.to_state();
        let xpub = {
            let tmp = state.try_borrow()?;
            tmp.xpub.to_owned()
        };

        let account = Bip44PublicAccount::<Secp256k1>::from_xpub(account, &xpub, network)?;
        Ok(Self { state, account, vault_dirty })
    }
}

impl Public {
    pub fn node(&self) -> &Bip32PublicNode<Secp256k1> {
        self.account.node()
    }

    pub fn bip44_path(&self) -> &Bip44AccountPath {
        self.account.bip44_path()
    }

    pub fn network(&self) -> &'static dyn Network<Suite = Secp256k1> {
        self.account.network()
    }

    // TODO: pub fn chain(&self, chain: Chain) -> Fallible<HydraPublicSubAccount>

    pub fn key(&mut self, idx: i32) -> Fallible<Bip44PublicKey<Secp256k1>> {
        touch_receive_idx(self.state.as_mut(), idx, self.vault_dirty.as_mut())?;
        self.account.key(idx)
    }

    pub fn key_by_p2pkh_addr(&self, addr: &str) -> Fallible<Bip44PublicKey<Secp256k1>> {
        // TODO include change addresses, too
        let receive_keys = self.receive_keys()?;
        for idx in 0..receive_keys {
            let key = self.account.key(idx as i32)?;
            if key.to_p2pkh_addr() == *addr {
                return Ok(key);
            }
        }
        bail!("Could not find {} among Hydra keys", addr)
    }

    pub fn xpub(&self) -> Fallible<String> {
        let state = self.state.try_borrow()?;
        Ok(state.xpub.to_owned())
    }

    pub fn receive_keys(&self) -> Fallible<u32> {
        let state = self.state.try_borrow()?;
        Ok(state.receive_keys)
    }

    pub fn change_keys(&self) -> Fallible<u32> {
        let state = self.state.try_borrow()?;
        Ok(state.change_keys)
    }

    pub(super) fn new(
        state: Box<dyn State<PublicState>>, account: Bip44PublicAccount<Secp256k1>,
        vault_dirty: Box<dyn State<bool>>,
    ) -> Self {
        Self { state, account, vault_dirty }
    }
}
