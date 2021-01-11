use super::*;

pub struct PrivateKind {
    state: Box<dyn State<Vec<String>>>,
    kind: MorpheusKind,
    vault_dirty: Box<dyn State<bool>>,
}

impl PrivateKind {
    pub(super) fn new(
        state: Box<dyn State<Vec<String>>>, kind: MorpheusKind, vault_dirty: Box<dyn State<bool>>,
    ) -> Self {
        Self { state, kind, vault_dirty }
    }

    pub fn path(&self) -> DidKind {
        self.kind.path()
    }

    pub fn node(&self) -> &Bip32Node<Ed25519> {
        &self.kind.node()
    }

    pub fn len(&self) -> Result<u32> {
        let state = self.state.try_borrow()?;
        Ok(state.len() as u32)
    }

    pub fn is_empty(&self) -> Result<bool> {
        let state = self.state.try_borrow()?;
        Ok(state.is_empty())
    }

    pub fn neuter(&self) -> PublicKind {
        PublicKind::new(self.state.clone(), self.kind.path())
    }

    pub fn key(&self, idx: i32) -> Result<MorpheusPrivateKey> {
        ensure!(idx >= 0, "Key index cannot be negative");
        let count = self.state.try_borrow()?.len() as i32;
        ensure!(idx < count, "Only existing keys can be queried");
        self.kind.key(idx)
    }

    pub fn key_mut(&mut self, idx: i32) -> Result<MorpheusPrivateKey> {
        ensure!(idx >= 0, "Key index cannot be negative");
        let count = self.state.try_borrow()?.len() as i32;
        let required = idx + 1;
        if count < required {
            let mut state = self.state.try_borrow_mut()?;
            for i in count..required {
                let pk = self.kind.key(i)?.neuter().public_key().to_string();
                state.push(pk)
            }
            let mut dirty = self.vault_dirty.try_borrow_mut()?;
            *dirty = true;
        }
        self.kind.key(idx)
    }

    pub fn key_by_pk(&self, pk: &MPublicKey) -> Result<MorpheusPrivateKey> {
        let count = self.state.try_borrow()?.len() as i32;
        for idx in 0..count {
            let persona = self.kind.key(idx)?;
            if persona.neuter().public_key() == *pk {
                return Ok(persona);
            }
        }
        bail!("Could not find {} among {:?} keys", pk, self.kind.path())
    }
}
