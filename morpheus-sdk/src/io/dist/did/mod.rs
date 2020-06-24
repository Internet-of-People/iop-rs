mod fake;
mod hydra;

use async_trait::async_trait;
use failure::Fallible;
use serde::{Deserialize, Serialize};

use crate::io::local::signer::MorpheusSigner;
use iop_morpheus_core::{
    crypto::{
        hash::{Content, ContentId},
        sign::{AfterProof, Signable, Signed},
    },
    data::{
        auth::Authentication,
        did::Did,
        diddoc::{BlockHeight, DidDocument, Right},
    },
};

pub use fake::FakeDidLedger;
pub use hydra::HydraDidLedger;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialOrd, PartialEq, Serialize)]
pub enum SignableOperationAttempt {
    AddKey,
    RevokeKey,
    AddRight,
    RevokeRight,
    TombstoneDid,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialOrd, PartialEq, Serialize)]
pub struct SignableOperationAttempts {
    attempts: Vec<SignableOperationAttempt>,
}

impl Content for SignableOperationAttempts {}
impl Signable for SignableOperationAttempts {}

// TODO add Hash after implemented for dependent types
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum OperationAttempt {
    RegisterBeforeProof,
    Signed(Signed<SignableOperationAttempts>),
}

#[async_trait(?Send)]
pub trait PooledLedgerTransaction {
    async fn ledger_status(&self) -> Fallible<Option<AfterProof>>;
    async fn morpheus_status(&self) -> Fallible<Option<bool>>;
}

// TODO change this trait to fetch full history as TimeSeries instead of latest snapshot
#[async_trait(?Send)]
pub trait LedgerQueries {
    async fn before_proof(&self, content: &ContentId) -> Fallible<Option<BlockHeight>>;
    async fn document(&self, did: &Did) -> Fallible<DidDocument>;
}

#[async_trait(?Send)]
pub trait LedgerOperations {
    async fn send_transaction(
        &self, operations: &[OperationAttempt],
    ) -> Fallible<Box<dyn PooledLedgerTransaction>>;
}

pub trait OwnDidDocumentFactory<T: OwnDidDocument, S: MorpheusSigner> {
    fn from(did: Did, signer: S) -> Fallible<T>;
}

#[async_trait(?Send)]
pub trait OwnDidDocument {
    fn did(&self) -> &Did;
    fn signer(&self) -> &dyn MorpheusSigner;

    async fn document(&self) -> Fallible<DidDocument>;

    // TODO should these operation be mutable?
    async fn add_key(
        &self, auth: &Authentication, expiry: Option<BlockHeight>,
    ) -> Fallible<SignableOperationAttempt>;
    async fn revoke_key(&self, auth: &Authentication) -> Fallible<SignableOperationAttempt>;
    async fn add_right(
        &self, auth: &Authentication, right: Right,
    ) -> Fallible<SignableOperationAttempt>;
    async fn revoke_right(
        &self, auth: &Authentication, right: Right,
    ) -> Fallible<SignableOperationAttempt>;
}
