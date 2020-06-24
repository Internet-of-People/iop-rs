use super::*;

pub struct Private {
    state: Box<dyn State<PublicState>>,
    root: MorpheusRoot,
    vault_dirty: Box<dyn State<bool>>,
}

impl PluginPrivate<Plugin> for Private {
    fn create(plugin: &Plugin, seed: Seed, vault_dirty: Box<dyn State<bool>>) -> Fallible<Self> {
        let root = Morpheus.root(&seed)?;
        let state = plugin.to_state();
        Ok(Private { state, root, vault_dirty })
    }
}

impl Private {
    pub fn personas(&self) -> Fallible<PrivateKind> {
        let state = State::map(self.state.as_ref(), |s| &s.personas, |s| &mut s.personas);
        let kind = self.root.personas()?;
        let vault_dirty = self.vault_dirty.clone();
        Ok(PrivateKind::new(state, kind, vault_dirty))
    }

    pub fn neuter(&self) -> Public {
        Public::new(self.state.clone())
    }

    pub fn key_by_pk(&self, pk: &MPublicKey) -> Fallible<MorpheusPrivateKey> {
        self.personas()?
            .key_by_pk(pk)
            .or_else(|_| bail!("Could not find {} among Morpheus keys", pk))
    }
}
