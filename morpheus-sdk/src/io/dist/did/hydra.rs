use async_trait::async_trait;
use failure::Fallible;
use serde::{Deserialize, Serialize};

use super::*;
use crate::crypto::{
    hash::{Content, ContentId},
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

#[async_trait(?Send)]
impl LedgerQueries for HydraDidLedger {
    async fn validate<T: Signable>(
        &self, on_behalf_of: &Did, signer_id: Option<MKeyId>, signed: &Signed<T>,
    ) -> Fallible<ValidationStatus> {
        todo!()
    }

    async fn validate_timeproofed<T: Signable>(
        &self, on_behalf_of: &Did, signer_id: Option<MKeyId>, signed: &AfterEnvelope<T>,
    ) -> Fallible<ValidationStatus> {
        todo!()
    }

    async fn before_proof_exists(&self, content: &ContentId) -> Fallible<bool> {
        todo!()
    }

    async fn document(&self, did: &Did) -> Fallible<DidDocument> {
        todo!()
    }
}

#[async_trait(?Send)]
impl LedgerOperations for HydraDidLedger {
    async fn send_transaction(
        &self, operations: &[OperationAttempt],
    ) -> Fallible<Box<PooledLedgerTransaction>> {
        todo!()
    }
}
