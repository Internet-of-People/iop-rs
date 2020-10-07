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

    pub fn validate(&self, parent_domain: &Domain, register: &DoRegister) -> Result<()> {
        match self {
            Self::Owner => {
                let child_owner = &register.owner;
                let parent_owner = parent_domain.owner();
                ensure!(
                    child_owner == parent_owner,
                    "Only {} can register a child of {}",
                    parent_owner,
                    parent_domain.name()
                );
            }
            Self::Any => {}
        };
        Ok(())
    }
}
