use super::*;

pub struct Private {
    plugin: Plugin,
    account: Bip44Account<Secp256k1>,
    vault_dirty: Box<dyn State<bool>>,
}

impl PluginPrivate<Plugin> for Private {
    fn create(plugin: &Plugin, seed: Seed, vault_dirty: Box<dyn State<bool>>) -> Fallible<Self> {
        let network = plugin.network();
        let account = plugin.account();

        let plugin = plugin.to_owned();
        let account = Bip44.network(&seed, network)?.account(account)?;
        Ok(Self { plugin, account, vault_dirty })
    }
}

impl Private {
    pub fn node(&self) -> &Bip32Node<Secp256k1> {
        self.account.node()
    }

    pub fn bip44_path(&self) -> &Bip44AccountPath {
        self.account.bip44_path()
    }

    pub fn network(&self) -> &'static dyn Network<Suite = Secp256k1> {
        self.account.network()
    }

    pub fn neuter(&self) -> Public {
        let plugin = self.plugin.to_owned();
        let account = self.account.neuter();
        let vault = self.vault_dirty.clone();
        Public::new(plugin, account, vault)
    }

    // TODO: pub fn chain(&self, chain: Chain) -> Fallible<HydraPrivateSubAccount>

    pub fn key(&mut self, idx: i32) -> Fallible<Bip44Key<Secp256k1>> {
        self.plugin.touch_receive_idx(idx, self.vault_dirty.as_mut())?;
        self.account.key(idx)
    }

    pub fn key_by_pk(&self, pk: &SecpPublicKey) -> Fallible<Bip44Key<Secp256k1>> {
        // TODO include change addresses, too
        let receive_keys = self.plugin.receive_keys();
        for idx in 0..receive_keys {
            let key = self.account.key(idx as i32)?;
            if key.neuter().to_public_key() == *pk {
                return Ok(key);
            }
        }
        bail!("Could not find {} among Hydra keys", pk)
    }
}
