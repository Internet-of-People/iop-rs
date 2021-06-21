use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionIdWithHeight {
    pub transaction_id: String,
    pub height: BlockHeight,
}

pub struct StateHolder {
    corrupted: bool,
    inner: Box<State>,
}

impl StateHolder {
    pub const CORRUPTED_ERR_MSG: &'static str =
        "Morpheus state is corrupt. All incoming changes will be ignored.";

    pub fn new() -> Self {
        Self { corrupted: false, inner: Default::default() }
    }

    pub fn is_corrupted(&self) -> bool {
        self.corrupted
    }

    pub fn ensure_not_corrupted(&self) -> Result<()> {
        ensure!(!self.corrupted, StateHolder::CORRUPTED_ERR_MSG);
        Ok(())
    }

    pub fn state(&self) -> Result<&State> {
        self.ensure_not_corrupted()?;
        Ok(&self.inner)
    }

    pub fn dry_run(&self, asset: &MorpheusAsset) -> Result<Vec<OperationError>> {
        if self.is_corrupted() {
            return Ok(vec![OperationError {
                invalid_operation_attempt: None,
                message: StateHolder::CORRUPTED_ERR_MSG.to_owned(),
            }]);
        }

        let temp = self.inner.clone();
        let res = asset.operation_attempts.iter().try_fold(
            temp,
            |mut inner, op| -> Result<Box<State>, OperationError> {
                inner.apply(Mutation::DoAttempt { op }).map_err(|e| OperationError {
                    invalid_operation_attempt: Some(op.clone()),
                    message: e.to_string(),
                })?;
                Ok(inner)
            },
        );
        // TODO The TS code just stopped on the 1st error, should we collect all?
        let errors = match res {
            Ok(_) => vec![],
            Err(e) => vec![e],
        };
        Ok(errors)
    }

    pub fn block_applying(&mut self, height: BlockHeight) -> Result<()> {
        self.ensure_not_corrupted()?;
        self.may_corrupt_state(|inner| inner.apply(Mutation::SetBlockHeight { height }))
    }

    pub fn apply_transaction(&mut self, txid: &str, asset: &MorpheusAsset) -> Result<()> {
        self.ensure_not_corrupted()?;
        asset
            .operation_attempts
            .iter()
            .try_for_each(|op| self.inner.apply(Mutation::RegisterAttempt { txid, op }))?;
        let inner_res = asset.operation_attempts.iter().try_fold(
            self.inner.clone(),
            |mut inner, op| -> Result<Box<State>> {
                inner.apply(Mutation::DoAttempt { op })?;
                Ok(inner)
            },
        );
        match inner_res {
            Ok(mut inner) => {
                inner.apply(Mutation::ConfirmTxn { txid })?;
                self.inner = inner;
                Ok(())
            }
            Err(e) => {
                self.inner.apply(Mutation::RejectTxn { txid })?;
                Err(e)
            }
        }
    }

    pub fn block_reverting(&mut self, height: BlockHeight) -> Result<()> {
        self.ensure_not_corrupted()?;
        self.may_corrupt_state(|inner| inner.revert(Mutation::SetBlockHeight { height }))
    }

    pub fn revert_transaction(&mut self, txid: &str, asset: &MorpheusAsset) -> Result<()> {
        self.ensure_not_corrupted()?;
        self.may_corrupt_state(|inner| {
            let confirmed_opt = inner.is_confirmed(txid);
            ensure!(
                confirmed_opt.is_some(),
                "Transaction {} has not been applied, cannot revert.",
                txid
            );

            // Option::unwrap is panic-free after handling None above
            if confirmed_opt.unwrap() {
                inner.revert(Mutation::ConfirmTxn { txid })?;
                asset.operation_attempts.iter().rev().try_for_each(|op| -> Result<()> {
                    inner.revert(Mutation::DoAttempt { op })?;
                    Ok(())
                })?;
            } else {
                inner.revert(Mutation::RejectTxn { txid })?;
            }
            asset.operation_attempts.iter().rev().try_for_each(|op| -> Result<()> {
                inner.revert(Mutation::RegisterAttempt { txid, op })?;
                Ok(())
            })?;
            Ok(())
        })
    }

    fn may_corrupt_state(&mut self, action: impl FnOnce(&mut State) -> Result<()>) -> Result<()> {
        if let Err(e) = action(&mut self.inner) {
            self.corrupted = true;
            Err(e)
        } else {
            Ok(())
        }
    }
}
