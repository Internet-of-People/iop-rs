use async_trait::async_trait;
use failure::Fallible;

use keyvault::multicipher::{MPublicKey, MSignature};
use morpheus_core::crypto::sign::SyncSigner;
use morpheus_core::{
    crypto::sign::{Signable, Signed},
    data::{
        auth::Authentication,
        claim::{WitnessRequest, WitnessStatement},
        present::ClaimPresentation,
    },
};

#[async_trait(?Send)]
pub trait Signer {
    fn authentication(&self) -> &Authentication;

    async fn sign(&self, data: &[u8]) -> Fallible<(MPublicKey, MSignature)>;

    async fn sign_witness_request(
        &self, request: WitnessRequest,
    ) -> Fallible<Signed<WitnessRequest>> {
        let content_to_sign = request.content_to_sign()?;
        let (public_key, signature) = self.sign(&content_to_sign).await?;
        Ok(Signed::new(public_key, request, signature))
    }

    async fn sign_witness_statement(
        &self, statement: WitnessStatement,
    ) -> Fallible<Signed<WitnessStatement>> {
        let content_to_sign = statement.content_to_sign()?;
        let (public_key, signature) = self.sign(&content_to_sign).await?;
        Ok(Signed::new(public_key, statement, signature))
    }

    async fn sign_claim_presentation(
        &self, presentation: ClaimPresentation,
    ) -> Fallible<Signed<ClaimPresentation>> {
        let content_to_sign = presentation.content_to_sign()?;
        let (public_key, signature) = self.sign(&content_to_sign).await?;
        Ok(Signed::new(public_key, presentation, signature))
    }
}

pub struct SyncAdapter<T: SyncSigner> {
    inner: T,
}

impl<T: SyncSigner> SyncAdapter<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[async_trait(?Send)]
impl<T: SyncSigner> Signer for SyncAdapter<T> {
    fn authentication(&self) -> &Authentication {
        self.inner.authentication()
    }

    async fn sign(&self, data: &[u8]) -> Fallible<(MPublicKey, MSignature)> {
        self.inner.sign(data)
    }
}
