use super::*;

pub struct PublicKind {
    state: Box<dyn State<Vec<String>>>,
    kind: DidKind,
}

impl PublicKind {
    pub(super) fn new(state: Box<dyn State<Vec<String>>>, kind: DidKind) -> Self {
        Self { state, kind }
    }

    pub fn path(&self) -> DidKind {
        self.kind
    }

    pub fn len(&self) -> Result<u32> {
        let state = self.state.try_borrow()?;
        Ok(state.len() as u32)
    }

    pub fn is_empty(&self) -> Result<bool> {
        let state = self.state.try_borrow()?;
        Ok(state.is_empty())
    }

    // In theory this could return Result<MorpheusPublicKey>, but that would need an EdExtPublicKey.
    // Since we do not have public derivation in ed25519, it is not worth the hassle.
    pub fn key(&self, idx: i32) -> Result<MPublicKey> {
        ensure!(idx >= 0, "Key index cannot be negative");
        let idx = idx as usize;
        let state = self.state.try_borrow()?;
        ensure!(idx < state.len(), "Only existing keys can be queried via Morpheus Public");
        let key: MPublicKey = state[idx].parse()?;
        Ok(key)
    }

    pub fn key_by_id(&self, id: &MKeyId) -> Result<MPublicKey> {
        let state = self.state.try_borrow()?;
        let count = state.len();
        for idx in 0..count {
            let key: MPublicKey = state[idx].parse()?;
            if key.validate_id(id) {
                return Ok(key);
            }
        }
        bail!("Could not find {} among {:?} keys", id, self.kind)
    }
}
