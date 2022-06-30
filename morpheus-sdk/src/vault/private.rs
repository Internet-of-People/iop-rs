use super::*;

pub struct Private {
    state: Box<dyn State<PublicState>>,
    root: MorpheusRoot,
    vault_dirty: Box<dyn State<bool>>,
}

impl PluginPrivate<Plugin> for Private {
    fn create(plugin: &Plugin, seed: Seed, vault_dirty: Box<dyn State<bool>>) -> Result<Self> {
        let root = Morpheus.root(&seed)?;
        let state = plugin.to_state();
        Ok(Private { state, root, vault_dirty })
    }
}

impl Private {
    pub fn kind(&self, did_kind: DidKind) -> Result<PrivateKind> {
        let state = <dyn State<_>>::map(
            self.state.as_ref(),
            PublicState::field_ref(did_kind),
            PublicState::field_mut(did_kind),
        );
        let kind = self.root.kind(did_kind)?;
        let vault_dirty = self.vault_dirty.clone();
        Ok(PrivateKind::new(state, kind, vault_dirty))
    }

    pub fn personas(&self) -> Result<PrivateKind> {
        self.kind(DidKind::Persona)
    }

    pub fn devices(&self) -> Result<PrivateKind> {
        self.kind(DidKind::Device)
    }

    pub fn groups(&self) -> Result<PrivateKind> {
        self.kind(DidKind::Group)
    }

    pub fn resources(&self) -> Result<PrivateKind> {
        self.kind(DidKind::Resource)
    }

    pub fn node(&self) -> &Bip32Node<Ed25519> {
        self.root.node()
    }

    pub fn public(&self) -> Public {
        Public::new(self.state.clone())
    }

    pub fn key_by_pk(&self, pk: &MPublicKey) -> Result<MorpheusPrivateKey> {
        for did_kind in DidKind::all() {
            let key_res = self.kind(*did_kind)?.key_by_pk(pk);
            if key_res.is_ok() {
                return key_res;
            }
        }
        bail!("Could not find {} among Morpheus keys", pk);
    }
}
