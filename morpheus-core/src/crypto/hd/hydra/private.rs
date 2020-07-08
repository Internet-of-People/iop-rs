use super::*;

use crate::hydra::crypto::HydraSigner;

pub struct Private {
    state: Box<dyn State<PublicState>>,
    account: Bip44Account<Secp256k1>,
    vault_dirty: Box<dyn State<bool>>,
}

impl PluginPrivate<Plugin> for Private {
    fn create(plugin: &Plugin, seed: Seed, vault_dirty: Box<dyn State<bool>>) -> Fallible<Self> {
        let network = plugin.network();
        let account = plugin.account();

        let state = plugin.to_state();
        let account = Bip44.network(&seed, network)?.account(account)?;
        Ok(Self { state, account, vault_dirty })
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

    pub fn public(&self) -> Public {
        let state = self.state.clone();
        let account = self.account.neuter();
        let vault = self.vault_dirty.clone();
        Public::new(state, account, vault)
    }

    // TODO: pub fn chain(&self, chain: Chain) -> Fallible<HydraPrivateSubAccount>

    pub fn key(&mut self, idx: i32) -> Fallible<Bip44Key<Secp256k1>> {
        touch_receive_idx(self.state.as_mut(), idx, self.vault_dirty.as_mut())?;
        self.account.key(idx)
    }

    pub fn key_by_pk(&self, pk: &SecpPublicKey) -> Fallible<Bip44Key<Secp256k1>> {
        // TODO include change addresses, too
        let state = self.state.try_borrow()?;
        let receive_keys = state.receive_keys;
        for idx in 0..receive_keys {
            let key = self.account.key(idx as i32)?;
            if key.neuter().to_public_key() == *pk {
                return Ok(key);
            }
        }
        bail!("Could not find {} among Hydra keys", pk)
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

    pub fn sign_hydra_transaction(
        &self, hyd_addr: &str, tx: &mut HydraTransactionData,
    ) -> Fallible<()> {
        let pub_key = self.public().key_by_p2pkh_addr(hyd_addr)?;
        let sk = self.key_by_pk(&pub_key.to_public_key())?;
        sk.to_private_key().sign_hydra_transaction(tx)
    }
}
