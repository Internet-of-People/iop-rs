use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DoTransfer {
    #[serde(with = "serde_str")]
    pub(super) name: DomainName,
    pub(super) to_owner: Principal,
}

impl AuthorizedCommand for DoTransfer {
    fn validate_auth(&self, state: &State, pk: &MPublicKey) -> Result<()> {
        state.validate_domain_owner(&self.name, pk)
    }
}

impl Command for DoTransfer {
    fn execute(self, state: &mut State) -> Result<UndoOperation> {
        ensure!(self.to_owner != Principal::system(), "Cannot transfer a domain to 'system'");
        let last_block = state.last_seen_height();

        let domain_mut = state.domain_mut(&self.name)?;
        ensure!(domain_mut.owner() != &Principal::system(), "Cannot transfer a system domain");
        ensure!(!domain_mut.is_expired_at(last_block), "Domain {} expired", self.name);

        let undo_operation = UndoTransfer { name: self.name, owner: domain_mut.owner().to_owned() };
        domain_mut.set_owner(self.to_owner);

        Ok(UndoOperation::Transfer(undo_operation))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UndoTransfer {
    #[serde(with = "serde_str")]
    pub(super) name: DomainName,
    pub(super) owner: Principal,
}

impl UndoCommand for UndoTransfer {
    fn execute(self, state: &mut State) -> Result<()> {
        let domain_mut = state.domain_mut(&self.name)?;
        domain_mut.set_owner(self.owner);
        Ok(())
    }
}
