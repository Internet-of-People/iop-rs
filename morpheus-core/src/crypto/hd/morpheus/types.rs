use super::*;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(super) struct Parameters {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicState {
    pub(super) personas: Vec<String>,
}
