use super::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RegistrationPolicy {
    Owner,
    Any,
}

impl Default for RegistrationPolicy {
    fn default() -> Self {
        Self::Owner
    }
}

impl RegistrationPolicy {
    pub fn any() -> Self {
        Self::Any
    }

    pub fn is_default(&self) -> bool {
        self == &Self::default()
    }
}
