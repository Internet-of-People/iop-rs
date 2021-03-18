use super::*;

pub trait RegistrationValidator {
    fn validate(&self, parent_domain: &Domain, register: &DoRegister) -> Result<()>;
}

impl RegistrationValidator for RegistrationPolicy {
    fn validate(&self, parent_domain: &Domain, register: &DoRegister) -> Result<()> {
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
