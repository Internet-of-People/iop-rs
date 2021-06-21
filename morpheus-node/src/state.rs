use iop_morpheus_proto::data::BeforeProofHistory;

use super::*;

pub(super) enum Mutation<'a> {
    SetBlockHeight { height: BlockHeight },
    RegisterAttempt { txid: &'a str, op: &'a OperationAttempt },
    DoAttempt { op: &'a OperationAttempt },
    ConfirmTxn { txid: &'a str },
    RejectTxn { txid: &'a str },
}

#[derive(Debug, Default, Clone)]
pub struct State {
    last_seen_height: BlockHeight,
    txn_status: HashMap<String, bool>,
    before_proofs: HashMap<String, BlockHeight>,
    did_states: HashMap<String, DidDocumentState>,
    did_txns: DidTransactionsState,
}

impl State {
    pub fn last_seen_height(&self) -> BlockHeight {
        self.last_seen_height
    }

    pub fn is_confirmed(&self, txid: &str) -> Option<bool> {
        self.txn_status.get(txid).cloned()
    }

    pub fn before_proof_exists_at(&self, content_id: &str, height: Option<BlockHeight>) -> bool {
        self.before_proofs
            .get(content_id)
            .map(|registered_at| height.map(|h| h >= *registered_at).unwrap_or(true))
            .unwrap_or(false)
    }

    pub fn before_proof_history(&self, content_id: &str) -> BeforeProofHistory {
        let exists_from_height = self.before_proofs.get(content_id).cloned();
        BeforeProofHistory {
            content_id: content_id.to_owned(),
            exists_from_height,
            queried_at_height: self.last_seen_height,
        }
    }

    pub fn get_tx_ids(
        &self, did: &str, include_attempts: bool, from_height_inc: BlockHeight,
        until_height_inc: Option<BlockHeight>,
    ) -> Option<impl Iterator<Item = &TransactionIdWithHeight>> {
        self.did_txns.get_between(did, from_height_inc, until_height_inc).map(move |i| {
            i.filter(move |t| {
                include_attempts || self.is_confirmed(&t.transaction_id).unwrap_or(false)
            })
        })
    }

    pub fn last_tx_id(&self, did: &str) -> Option<&TransactionIdWithHeight> {
        self.get_tx_ids(did, false, 0, None).and_then(|mut i| i.next())
    }

    pub fn get_doc_at(
        &self, did_data: &str, height_opt: Option<BlockHeight>,
    ) -> Result<DidDocument> {
        let height = height_opt.unwrap_or(self.last_seen_height);
        let did: Did = did_data.parse()?;
        let default_state = DidDocumentState::new(&did);
        let state = self.did_states.get(did_data).unwrap_or(&default_state);
        let doc = state.at_height(&did, height)?;
        Ok(doc)
    }

    fn did_state_mut(
        &mut self, did: &Did, last_tx_id: &Option<String>,
    ) -> Result<&mut DidDocumentState> {
        let height = self.last_seen_height;
        let did_data = did.to_string();

        let chain_last_txn = self.last_tx_id(&did_data);
        if last_tx_id.as_ref() != chain_last_txn.map(|t| &t.transaction_id) {
            let op_state = if let Some(txid) = last_tx_id {
                format!("after txn {}", txid)
            } else {
                "on an implicit document".to_owned()
            };
            let chain_state = if let Some(txn) = chain_last_txn {
                format!(
                    "but it last changed at height {} by txn {}",
                    txn.height, txn.transaction_id
                )
            } else {
                "but it never changed yet".to_owned()
            };
            bail!(
                "Operation on {} at height {} was attempted {}, {}",
                &did_data,
                height,
                op_state,
                chain_state
            )
        }

        let state =
            self.did_states.entry(did_data.clone()).or_insert_with(|| DidDocumentState::new(&did));

        Ok(state)
    }

    pub(super) fn apply(&mut self, mutation: Mutation) -> Result<()> {
        fn insert_txn_status(this: &mut State, txid: &str, status: bool) -> Result<()> {
            // We can change the state fearlessly even if we Err, because the caller will throw away changed state on error
            ensure!(
                this.txn_status.insert(txid.to_owned(), status).is_none(),
                "Transaction {} was already confirmed",
                txid
            );
            Ok(())
        }

        fn insert_did_txns(this: &mut State, txid: &str, signed_op: &SignedOperation) {
            signed_op.attempts_unsafe_without_signature_checking().for_each(|op| {
                let item = DidTransactionItem {
                    did: op.did.to_string(),
                    txid,
                    height: this.last_seen_height,
                };
                this.did_txns.apply(item);
            })
        }

        fn insert_before_proof(
            this: &mut State, content_id: &str, height: BlockHeight,
        ) -> Result<()> {
            // We can change the state fearlessly even if we Err, because the caller will throw away changed state on error
            if let Some(old_height) = this.before_proofs.insert(content_id.to_owned(), height) {
                bail!("Before proof {} is already registered at {}", content_id, old_height)
            }
            Ok(())
        }

        fn check_state(
            state: &mut DidDocumentState, did: &Did, height: u32, signer: &Authentication,
        ) -> Result<()> {
            let did_data = did.to_string();

            let doc = state.at_height(did, height)?;
            let tombstoned = doc.is_tombstoned_at(height)?;
            let can_update = doc.has_right_at(signer, Right::Update, height)?;

            ensure!(
                !tombstoned,
                "{} cannot update {} at height {}. The DID is tombstoned",
                signer,
                &did_data,
                height
            );

            ensure!(
                can_update,
                "{} has no right to update {} at height {}",
                signer,
                &did_data,
                height
            );

            Ok(())
        }

        fn apply_signed_op(this: &mut State, op: &SignedOperation) -> Result<()> {
            op.attempts()?.try_for_each(|a| -> Result<()> {
                let signer = Authentication::PublicKey(op.signer_public_key.parse()?);
                let height = this.last_seen_height;
                let state = this.did_state_mut(&a.did, &a.last_tx_id)?;
                check_state(state, &a.did, height, &signer)?;
                state.apply(&a.did, height, &signer, &a.operation)
            })
        }

        use Mutation::*;
        match mutation {
            SetBlockHeight { height } => {
                ensure!(
                    self.last_seen_height <= height,
                    "The applied height ({}) is < last seen height ({})",
                    height,
                    self.last_seen_height
                );
                self.last_seen_height = height;
            }
            RegisterAttempt { txid, op } => {
                if let OperationAttempt::Signed(signed_op) = op {
                    insert_did_txns(self, txid, signed_op);
                }
            }
            DoAttempt { op } => match op {
                OperationAttempt::RegisterBeforeProof { content_id } => {
                    insert_before_proof(self, content_id, self.last_seen_height)?
                }
                OperationAttempt::Signed(op) => apply_signed_op(self, op)?,
            },
            ConfirmTxn { txid } => insert_txn_status(self, txid, true)?,
            RejectTxn { txid } => insert_txn_status(self, txid, false)?,
        }
        Ok(())
    }

    pub(super) fn revert(&mut self, mutation: Mutation) -> Result<()> {
        fn remove_txn_status(this: &mut State, txid: &str, status: bool) -> Result<()> {
            let confirmed_opt = this.txn_status.remove(txid);
            ensure!(confirmed_opt.is_some(), "Transaction {} was not seen", txid);
            if confirmed_opt.unwrap() {
                ensure!(
                    status,
                    "Transaction {} was confirmed, hence its rejection cannot be reverted",
                    txid
                );
            } else {
                ensure!(
                    !status,
                    "Transaction {} was rejected, hence its confirmation cannot be reverted",
                    txid
                );
            }
            Ok(())
        }

        fn remove_did_txns(this: &mut State, txid: &str, signed_op: &SignedOperation) {
            signed_op.attempts_unsafe_without_signature_checking().rev().for_each(|op| {
                let item = DidTransactionItem {
                    did: op.did.to_string(),
                    txid,
                    height: this.last_seen_height,
                };
                this.did_txns.revert(item);
            })
        }

        fn remove_before_proof(this: &mut State, content_id: &str) -> Result<()> {
            let height = this.last_seen_height;
            if let Some(old_height) = this.before_proofs.remove(content_id) {
                ensure!(
                    height == old_height,
                    "Before proof {} was registered at {}, cannot be reverted at {}",
                    content_id,
                    old_height,
                    height
                );
                Ok(())
            } else {
                bail!(
                    "Before proof {} was not registered, therefore cannot be reverted",
                    content_id
                );
            }
        }

        fn revert_signed_op(this: &mut State, op: &SignedOperation) -> Result<()> {
            op.attempts()?.rev().try_for_each(|a| -> Result<()> {
                let signer = Authentication::PublicKey(op.signer_public_key.parse()?);
                let height = this.last_seen_height;
                let state = this.did_state_mut(&a.did, &a.last_tx_id)?;
                state.revert(&a.did, height, &signer, &a.operation)
            })
        }

        use Mutation::*;
        match mutation {
            SetBlockHeight { height } => {
                ensure!(
                    self.last_seen_height >= height,
                    "The reverted height ({}) is > last seen height ({})",
                    height,
                    self.last_seen_height
                );
                self.last_seen_height = height;
            }
            RegisterAttempt { txid, op } => {
                if let OperationAttempt::Signed(signed_op) = op {
                    remove_did_txns(self, txid, signed_op);
                }
            }
            DoAttempt { op } => match op {
                OperationAttempt::RegisterBeforeProof { content_id } => {
                    remove_before_proof(self, content_id)?
                }
                OperationAttempt::Signed(op) => revert_signed_op(self, op)?,
            },
            ConfirmTxn { txid } => remove_txn_status(self, txid, true)?,
            RejectTxn { txid } => remove_txn_status(self, txid, false)?,
        }
        Ok(())
    }
}
