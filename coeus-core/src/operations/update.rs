use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DoUpdate {
    pub(super) name: DomainName,
    pub(super) data: DynamicContent,
}

impl AuthorizedCommand for DoUpdate {
    fn validate_auth(&self, state: &State, pk: &MPublicKey) -> Result<()> {
        state.validate_domain_owner(&self.name, pk)
    }
}

impl Command for DoUpdate {
    fn execute(self, state: &mut State) -> Result<UndoOperation> {
        let last_block = state.last_seen_height();

        let domain_mut = state.domain_mut(&self.name)?;
        ensure!(!domain_mut.is_expired_at(last_block), "Domain {} expired", self.name);

        let mut undo_operation = UndoUpdate { name: self.name, data: self.data };
        std::mem::swap(&mut undo_operation.data, domain_mut.data_mut());

        match state.validate_subtree_policies(&undo_operation.name) {
            Ok(()) => Ok(UndoOperation::Update(undo_operation)),
            Err(e) => {
                undo_operation.execute(state)?;
                Err(e)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UndoUpdate {
    name: DomainName,
    data: DynamicContent,
}

impl UndoCommand for UndoUpdate {
    fn execute(self, state: &mut State) -> Result<()> {
        let domain = state.domain_mut(&self.name)?;
        domain.set_data(self.data);
        Ok(())
    }
}
