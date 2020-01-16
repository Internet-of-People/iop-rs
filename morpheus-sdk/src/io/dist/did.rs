use async_trait::async_trait;
use failure::Fallible;
use serde::{Deserialize, Serialize};

use crate::crypto::{
    hash::{Content, ContentId},
    sign::{AfterEnvelope, AfterProof, BlockHash, Signable, Signed, Signer},
};
use crate::data::{
    did::Did,
    diddoc::{Authentication, BlockHeight, DidDocument, Right},
};
use keyvault::multicipher::MKeyId;

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
pub trait DidDocumentLedgerQueries {
    async fn validate<T: Signable>(
        &self, on_behalf_of: &Did, signer_id: Option<MKeyId>, signed: &Signed<T>,
    ) -> Fallible<ValidationStatus>;

    async fn validate_timeproofed<T: Signable>(
        &self, on_behalf_of: &Did, signer_id: Option<MKeyId>, signed: &AfterEnvelope<T>,
    ) -> Fallible<ValidationStatus>;

    async fn before_proof_exists(&self, content: &ContentId) -> Fallible<bool>;
    async fn document(&self, did: &Did) -> Fallible<DidDocument>;
}

#[async_trait(?Send)]
pub trait DidDocumentLedgerOperations {
    async fn send_transaction(
        &self, operations: &[OperationAttempt],
    ) -> Fallible<Box<PooledLedgerTransaction>>;
}

pub struct HydraDidLedger {
    // TODO
}

#[async_trait(?Send)]
impl DidDocumentLedgerQueries for HydraDidLedger {
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
impl DidDocumentLedgerOperations for HydraDidLedger {
    async fn send_transaction(
        &self, operations: &[OperationAttempt],
    ) -> Fallible<Box<PooledLedgerTransaction>> {
        todo!()
    }
}

// TODO consider if this is needed in general? We could do this with a From-like wrapper trait.
pub type OwnDidDocumentFactory<T: OwnDidDocument> = FnOnce(Did, &dyn Signer) -> Fallible<T>;

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
