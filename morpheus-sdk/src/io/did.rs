use async_trait::async_trait;
use failure::Fallible;

use crate::data::{
    did::Did,
    diddoc::{Authentication, BlockHeight, DidDocument, Right},
    sign::{AfterEnvelope, Signable, Signed},
};
use keyvault::multicipher;

pub enum ValidationResult {
    /// All possible checks are done and passed.
    Valid,
    /// Some checks could not be performed for lack of information, all others passed.
    /// E.g. Signatures are valid, but no timestamps are present so
    /// they could have been created outside the time period in which the signer key was valid.
    MaybeValid,
    /// Any step of validation failed.
    Invalid,
}

#[async_trait(?Send)]
trait DidNet {
    async fn validate<T: Signable>(
        &self, on_behalf_of: &Did, signer_key: Option<multicipher::MKeyId>,
        signed_content: &Signed<T>,
    ) -> Fallible<ValidationResult>;

    async fn validate_timeproofed<T: Signable>(
        &self, on_behalf_of: &Did, signer_key: Option<multicipher::MKeyId>,
        signed_content: &AfterEnvelope<T>,
    ) -> Fallible<ValidationResult>;

    async fn get_document(&self, did: &Did) -> Fallible<DidDocument>;

    async fn add_key(&self, auth: &Authentication, expiry: Option<BlockHeight>) -> Fallible<()>;
    async fn revoke_key(&self, auth: &Authentication) -> Fallible<()>;
    async fn add_right(&self, auth: &Authentication, right: Right) -> Fallible<()>;
    async fn revoke_right(&self, auth: &Authentication, right: Right) -> Fallible<()>;
}
