use async_trait::async_trait;
use failure::Fallible;

use super::*;
use crate::crypto::{
    hash::ContentId,
    sign::{AfterEnvelope, AfterProof, BlockHash, Signable, Signed, Signer},
};
use crate::data::{
    did::Did,
    diddoc::{Authentication, BlockHeight, DidDocument, Right},
};
use keyvault::multicipher::MKeyId;

pub struct HydraDidLedger {
    // TODO
}

impl HydraDidLedger {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait(?Send)]
impl LedgerQueries for HydraDidLedger {
    async fn validate<T: Signable>(
        &self, _on_behalf_of: &Did, _signer_id: Option<MKeyId>, _signed: &Signed<T>,
    ) -> Fallible<ValidationStatus> {
        todo!()
    }

    async fn validate_timeproofed<T: Signable>(
        &self, _on_behalf_of: &Did, _signer_id: Option<MKeyId>, _signed: &AfterEnvelope<T>,
    ) -> Fallible<ValidationStatus> {
        todo!()
    }

    async fn before_proof_exists(&self, _content: &ContentId) -> Fallible<bool> {
        todo!()
    }

    async fn document(&self, _did: &Did) -> Fallible<DidDocument> {
        todo!()
    }
}

#[async_trait(?Send)]
impl LedgerOperations for HydraDidLedger {
    async fn send_transaction(
        &self, _operations: &[OperationAttempt],
    ) -> Fallible<Box<dyn PooledLedgerTransaction>> {
        todo!()
    }
}
