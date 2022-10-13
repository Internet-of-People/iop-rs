use super::*;

impl AuthorizedCommand for DoRegister {
    // Note: this warning reminds us that in order to let DIDs to own domains, we'll need this state.
    fn validate_auth(&self, state: &State, pk: &MPublicKey) -> Result<()> {
        self.owner.validate_impersonation(pk)
    }
}

fn validate_inside_state(
    state: &State, name: &DomainName, domain_before_op_opt: &Option<Domain>,
) -> Result<()> {
    if let Some(domain_before_op) = domain_before_op_opt {
        let last_block = state.last_seen_height();
        ensure!(domain_before_op.is_expired_at(last_block), "Valid domain {} exists", name);
        ensure!(
            domain_before_op.is_grace_period_over(last_block),
            "Expired domain {} in grace period exists",
            name
        );
    }

    state.validate_subtree_policies(name)
}

impl Command for DoRegister {
    fn execute(self, state: &mut State) -> Result<UndoOperation> {
        let parent_name = self.name.parent().with_context(|| "Cannot register root domain")?;
        let parent_domain = state.domain_mut(&parent_name)?;
        parent_domain.registration_policy().validate(parent_domain, &self)?;

        // NOTE child domain must be inserted into state so that
        //      it can be reached by domain::child(edge) -> domain queries during schema validation
        let child_domain = Domain::new(
            self.name.to_owned(),
            self.owner,
            self.subtree_policies,
            self.registration_policy,
            self.data,
            self.expires_at_height,
        );
        let old_domain = parent_domain.insert_or_replace_child(child_domain)?;
        let undo_operation = UndoRegister { name: self.name.to_owned(), old_domain };

        match validate_inside_state(state, &self.name, &undo_operation.old_domain) {
            Ok(()) => Ok(UndoOperation::Register(Box::new(undo_operation))),
            Err(e) => {
                undo_operation.execute(state)?;
                Err(e)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UndoRegister {
    #[serde(with = "serde_str")]
    name: DomainName,
    old_domain: Option<Domain>,
}

impl UndoCommand for UndoRegister {
    fn execute(self, state: &mut State) -> Result<()> {
        let parent_name =
            self.name.parent().with_context(|| "Cannot undo registering root domain")?;
        let parent_domain = state.domain_mut(&parent_name)?;
        match self.old_domain {
            Some(old) => {
                parent_domain.insert_or_replace_child(old)?;
            }
            None => {
                parent_domain.remove_child(self.name.last_edge().unwrap())?;
            }
        };

        Ok(())
    }
}
