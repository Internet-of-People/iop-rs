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

    pub fn len(&self) -> Fallible<usize> {
        let state = self.state.try_borrow()?;
        Ok(state.len())
    }

    pub fn is_empty(&self) -> Fallible<bool> {
        let state = self.state.try_borrow()?;
        Ok(state.is_empty())
    }

    pub fn key(&self, idx: i32) -> Fallible<MPublicKey> {
        ensure!(idx >= 0, "Key index cannot be negative");
        let idx = idx as usize;
        let state = self.state.try_borrow()?;
        ensure!(idx < state.len());
        let key: MPublicKey = state[idx].parse()?;
        Ok(key)
    }

    pub fn key_by_id(&self, id: &MKeyId) -> Fallible<MPublicKey> {
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
