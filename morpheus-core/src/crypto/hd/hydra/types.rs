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
