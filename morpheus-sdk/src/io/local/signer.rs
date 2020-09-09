use super::*;

use iop_keyvault::multicipher::{MPublicKey, MSignature};
use iop_morpheus_core::{
    crypto::sign::{Signable, Signed, SyncMorpheusSigner},
    data::{
        claim::{WitnessRequest, WitnessStatement},
        present::ClaimPresentation,
    },
};

#[async_trait(?Send)]
pub trait MorpheusSigner {
    async fn sign(&self, data: &[u8]) -> Result<(MPublicKey, MSignature)>;

    async fn sign_witness_request(
        &self, request: WitnessRequest,
    ) -> Result<Signed<WitnessRequest>> {
        let content_to_sign = request.content_to_sign()?;
        let (public_key, signature) = self.sign(&content_to_sign).await?;
        Ok(Signed::new(public_key, request, signature))
    }

    async fn sign_witness_statement(
        &self, statement: WitnessStatement,
    ) -> Result<Signed<WitnessStatement>> {
        let content_to_sign = statement.content_to_sign()?;
        let (public_key, signature) = self.sign(&content_to_sign).await?;
        Ok(Signed::new(public_key, statement, signature))
    }

    async fn sign_claim_presentation(
        &self, presentation: ClaimPresentation,
    ) -> Result<Signed<ClaimPresentation>> {
        let content_to_sign = presentation.content_to_sign()?;
        let (public_key, signature) = self.sign(&content_to_sign).await?;
        Ok(Signed::new(public_key, presentation, signature))
    }
}

pub struct SyncAdapter<T: SyncMorpheusSigner> {
    inner: T,
}

impl<T: SyncMorpheusSigner> SyncAdapter<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[async_trait(?Send)]
impl<T: SyncMorpheusSigner> MorpheusSigner for SyncAdapter<T> {
    async fn sign(&self, data: &[u8]) -> Result<(MPublicKey, MSignature)> {
        self.inner.sign(data)
    }
}
