use super::*;

pub struct Public {
    state: Box<dyn State<PublicState>>,
}

impl PluginPublic<Plugin> for Public {
    fn create(plugin: &Plugin, _vault_dirty: Box<dyn State<bool>>) -> Result<Self> {
        Ok(Self { state: plugin.to_state() })
    }
}

impl Public {
    pub(super) fn new(state: Box<dyn State<PublicState>>) -> Self {
        Self { state }
    }

    pub fn kind(&self, did_kind: DidKind) -> Result<PublicKind> {
        let state = <dyn State<_>>::map(
            self.state.as_ref(),
            PublicState::field_ref(did_kind),
            PublicState::field_mut(did_kind),
        );
        Ok(PublicKind::new(state, did_kind))
    }

    pub fn personas(&self) -> Result<PublicKind> {
        self.kind(DidKind::Persona)
    }

    pub fn devices(&self) -> Result<PublicKind> {
        self.kind(DidKind::Device)
    }

    pub fn groups(&self) -> Result<PublicKind> {
        self.kind(DidKind::Group)
    }

    pub fn resources(&self) -> Result<PublicKind> {
        self.kind(DidKind::Resource)
    }

    pub fn key_by_id(&self, id: &MKeyId) -> Result<MPublicKey> {
        for did_kind in DidKind::all() {
            let key_res = self.kind(*did_kind)?.key_by_id(id);
            if key_res.is_ok() {
                return key_res;
            }
        }
        bail!("Could not find {} among Morpheus key ids", id);
    }
}
