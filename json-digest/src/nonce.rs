use super::*;

/// Multibase-encoded random content, e.g. 'urvU8F6HmEol5zOmHh_nnS1RiX5r3T2t9U_d_kQY7ZC-I"
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Nonce264(pub String);

impl Nonce264 {
    /// Generates a new Nonce. Uses the getrandom crate to find the best source of entropy on the platform.
    /// In JavaScript tests you might need to refer to https://github.com/jsdom/jsdom/issues/1612 for
    /// how to fix phantom browsers to comply with HTML5 specs.
    pub fn generate() -> Self {
        use rand::{thread_rng, RngCore};
        let mut arr = [0u8; 33];
        thread_rng().fill_bytes(&mut arr[..]);
        let encoded = multibase::encode(multibase::Base::Base64Url, &arr[..]);
        Self(encoded)
    }
}
