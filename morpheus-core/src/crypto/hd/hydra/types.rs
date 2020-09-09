use super::*;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Parameters {
    pub(super) network: String,
    pub(super) account: i32,
}

impl Parameters {
    pub fn new(network: &'static dyn Network<Suite = Secp256k1>, account: i32) -> Self {
        Self { network: network.name().to_string(), account }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicState {
    pub(super) xpub: String,
    pub(super) receive_keys: u32,
    pub(super) change_keys: u32, // TODO there is no way for creating change keys for now
}

pub(super) fn touch_receive_idx(
    state: &mut dyn State<PublicState>, idx: i32, vault_dirty: &mut dyn State<bool>,
) -> Result<()> {
    ensure!(idx >= 0, "Key index cannot be negative");
    let required_keys = (idx as u32) + 1;
    let receive_keys = {
        let state = state.try_borrow()?;
        state.receive_keys
    };
    if required_keys > receive_keys {
        let mut state = state.try_borrow_mut()?;
        let mut dirty = vault_dirty.try_borrow_mut()?;
        state.receive_keys = required_keys;
        *dirty = true;
    }
    Ok(())
}
