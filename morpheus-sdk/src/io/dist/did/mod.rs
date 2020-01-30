mod fake;
mod hydra;

use async_trait::async_trait;
use failure::Fallible;
use serde::{Deserialize, Serialize};

use crate::crypto::{
    hash::{Content, ContentId},
    sign::{AfterProof, Signable, Signed, Signer},
};
use crate::data::{
    auth::Authentication,
    did::Did,
    diddoc::{BlockHeight, DidDocument, Right},
};

pub use fake::FakeDidLedger;
pub use hydra::HydraDidLedger;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialOrd, PartialEq, Serialize)]
pub enum ValidationStatus {
    /// All possible checks are done and passed.
    Valid,
    /// Some checks could not be performed for lack of information, all others passed.
    /// E.g. Signatures are valid, but no timestamps are present so
    /// they could have been created outside the time period in which the signer key was valid.
    MaybeValid,
    /// Any step of validation failed.
    Invalid,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialOrd, PartialEq, Serialize)]
pub enum ValidationIssueSeverity {
    Warning,
    Error,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialOrd, PartialEq, Serialize)]
pub struct ValidationIssue {
    code: u32,
    reason: String,
    severity: ValidationIssueSeverity,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialOrd, PartialEq, Serialize)]
pub struct ValidationResult {
    issues: Vec<ValidationIssue>,
}

impl ValidationResult {
    pub fn status(&self) -> ValidationStatus {
        let has_error = self.issues.iter().any(|it| it.severity == ValidationIssueSeverity::Error);
        if has_error {
            ValidationStatus::Invalid
        } else if !self.issues.is_empty() {
            ValidationStatus::MaybeValid
        } else {
            ValidationStatus::Valid
        }
    }
}

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
    RevokeBeforeProof,
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

pub trait OwnDidDocumentFactory<T: OwnDidDocument, S: Signer> {
    fn from(did: Did, signer: S) -> Fallible<T>;
}

#[async_trait(?Send)]
pub trait OwnDidDocument {
    fn did(&self) -> &Did;
    fn signer(&self) -> &dyn Signer;

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
