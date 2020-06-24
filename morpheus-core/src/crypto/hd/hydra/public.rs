use super::*;

pub struct Public {
    plugin: Plugin,
    account: Bip44PublicAccount<Secp256k1>,
    vault_dirty: Box<dyn State<bool>>,
}

impl PluginPublic<Plugin> for Public {
    fn create(plugin: &Plugin, vault_dirty: Box<dyn State<bool>>) -> Fallible<Self> {
        let network = plugin.network();
        let account = plugin.account();
        let xpub = plugin.xpub();

        let plugin = plugin.to_owned();
        let account = Bip44PublicAccount::<Secp256k1>::from_xpub(account, &xpub, network)?;
        Ok(Self { plugin, account, vault_dirty })
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
        self.plugin.touch_receive_idx(idx, self.vault_dirty.as_mut())?;
        self.account.key(idx)
    }

    // pub fn key_by_id(&self, pk: &SecpKeyId) -> Fallible<SecpPublicKey> {
    //     // TODO include change addresses, too
    //     let receive_keys = self.plugin.receive_keys();
    //     for idx in 0..receive_keys {
    //         let key = self.account.key(idx as i32)?;
    //         if key.neuter().to_public_key() == *pk {
    //             return Ok(key);
    //         }
    //     }
    //     bail!("Could not find {} among Hydra keys", pk)
    // }

    pub(super) fn new(
        plugin: Plugin, account: Bip44PublicAccount<Secp256k1>, vault_dirty: Box<dyn State<bool>>,
    ) -> Self {
        Self { plugin, account, vault_dirty }
    }
}
