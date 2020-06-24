use super::*;

pub struct Public {
    state: Box<dyn State<PublicState>>,
}

impl PluginPublic<Plugin> for Public {
    fn create(plugin: &Plugin, _vault_dirty: Box<dyn State<bool>>) -> Fallible<Self> {
        Ok(Self { state: plugin.to_state() })
    }
}

impl Public {
    pub(super) fn new(state: Box<dyn State<PublicState>>) -> Self {
        Self { state }
    }

    pub fn personas(&self) -> Fallible<PublicKind> {
        let state = State::map(self.state.as_ref(), |s| &s.personas, |s| &mut s.personas);
        Ok(PublicKind::new(state, DidKind::Persona))
    }

    pub fn key_by_id(&self, id: &MKeyId) -> Fallible<MPublicKey> {
        self.personas()?
            .key_by_id(id)
            .or_else(|_| bail!("Could not find {} among Morpheus keys", id))
    }
}
