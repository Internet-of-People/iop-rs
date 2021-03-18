use super::*;

impl AuthorizedCommand for DoRenew {
    fn validate_auth(&self, state: &State, pk: &MPublicKey) -> Result<()> {
        state.validate_domain_owner(&self.name, pk)
    }
}

impl Command for DoRenew {
    // TODO check expiration policies
    fn execute(self, state: &mut State) -> Result<UndoOperation> {
        let domain = state.domain_mut(&self.name)?;

        let undo_operation =
            UndoRenew { name: self.name, expires_at_height: domain.expires_at_height() };
        domain.set_expires_at_height(self.expires_at_height);
        Ok(UndoOperation::Renew(undo_operation))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UndoRenew {
    #[serde(with = "serde_str")]
    name: DomainName,
    expires_at_height: BlockHeight,
}

impl UndoCommand for UndoRenew {
    fn execute(self, state: &mut State) -> Result<()> {
        let domain = state.domain_mut(&self.name)?;
        domain.set_expires_at_height(self.expires_at_height);
        Ok(())
    }
}
