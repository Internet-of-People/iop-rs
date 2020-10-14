use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DoDelete {
    pub(super) name: DomainName,
}

impl AuthorizedCommand for DoDelete {
    fn validate_auth(&self, state: &State, pk: &MPublicKey) -> Result<()> {
        state.validate_domain_owner(&self.name, pk)
    }
}

impl Command for DoDelete {
    fn execute(self, state: &mut State) -> Result<UndoOperation> {
        let parent_name = self.name.parent().with_context(|| "Cannot delete root domain")?;
        let parent_domain = state.domain_mut(&parent_name)?;
        let child_edge = self.name.last_edge().unwrap();
        // NOTE delete is allowed for expired domains and is essentially a no-op if grace period ended anyway
        let domain = parent_domain.remove_child(child_edge)?;
        let undo_operation = UndoDelete { domain };
        Ok(UndoOperation::Delete(undo_operation))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UndoDelete {
    domain: Domain,
}

impl UndoCommand for UndoDelete {
    fn execute(self, state: &mut State) -> Result<()> {
        let parent_name =
            self.domain.name().parent().with_context(|| "Cannot undo deleting root domain")?;
        let parent_domain = state.domain_mut(&parent_name)?;
        parent_domain.insert_or_replace_child(self.domain)?;
        Ok(())
    }
}
