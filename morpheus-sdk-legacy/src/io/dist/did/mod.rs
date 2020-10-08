mod fake;
mod hydra;

use super::*;

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
    async fn ledger_status(&self) -> Result<Option<AfterProof>>;
    async fn morpheus_status(&self) -> Result<Option<bool>>;
}

// TODO change this trait to fetch full history as TimeSeries instead of latest snapshot
#[async_trait(?Send)]
pub trait LedgerQueries {
    async fn before_proof(&self, content: &ContentId) -> Result<Option<BlockHeight>>;
    async fn document(&self, did: &Did) -> Result<DidDocument>;
}

#[async_trait(?Send)]
pub trait LedgerOperations {
    async fn send_transaction(
        &self, operations: &[OperationAttempt],
    ) -> Result<Box<dyn PooledLedgerTransaction>>;
}

pub trait OwnDidDocumentFactory<T: OwnDidDocument, S: MorpheusSigner> {
    fn from(did: Did, signer: S) -> Result<T>;
}

#[async_trait(?Send)]
pub trait OwnDidDocument {
    fn did(&self) -> &Did;
    fn signer(&self) -> &dyn MorpheusSigner;

    async fn document(&self) -> Result<DidDocument>;

    // TODO should these operation be mutable?
    async fn add_key(
        &self, auth: &Authentication, expiry: Option<BlockHeight>,
    ) -> Result<SignableOperationAttempt>;
    async fn revoke_key(&self, auth: &Authentication) -> Result<SignableOperationAttempt>;
    async fn add_right(
        &self, auth: &Authentication, right: Right,
    ) -> Result<SignableOperationAttempt>;
    async fn revoke_right(
        &self, auth: &Authentication, right: Right,
    ) -> Result<SignableOperationAttempt>;
}
