use async_trait::async_trait;
use failure::Fallible;

use keyvault::{
    multicipher::{MPrivateKey, MPublicKey, MSignature},
    PrivateKey,
};
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

    // TODO MUST BE CHANGED to have separated special-purpose signer functions
    //      so that the user can receive a confirmation dialog with relevant semantics
    //      fn sign(&self, req: &WitnessRequest) -> Fallible<Signed<WitnessRequest>>
    async fn sign(&self, data: Vec<u8>) -> Fallible<(MPublicKey, MSignature)>;

    async fn sign_witness_request(
        &self, request: WitnessRequest,
    ) -> Fallible<Signed<WitnessRequest>> {
        let content_to_sign = request.content_to_sign()?;
        let (public_key, signature) = self.sign(content_to_sign).await?;
        Ok(Signed::new(public_key, request, signature))
    }

    async fn sign_witness_statement(
        &self, statement: WitnessStatement,
    ) -> Fallible<Signed<WitnessStatement>> {
        let content_to_sign = statement.content_to_sign()?;
        let (public_key, signature) = self.sign(content_to_sign).await?;
        Ok(Signed::new(public_key, statement, signature))
    }

    async fn sign_claim_presentation(
        &self, presentation: ClaimPresentation,
    ) -> Fallible<Signed<ClaimPresentation>> {
        let content_to_sign = presentation.content_to_sign()?;
        let (public_key, signature) = self.sign(content_to_sign).await?;
        Ok(Signed::new(public_key, presentation, signature))
    }
}

pub struct PrivateKeySigner {
    private_key: MPrivateKey,
    authentication: Authentication,
}

impl PrivateKeySigner {
    pub fn new(private_key: MPrivateKey, authentication: Authentication) -> Self {
        Self { private_key, authentication }
    }
}

#[async_trait(?Send)]
impl Signer for PrivateKeySigner {
    fn authentication(&self) -> &Authentication {
        &self.authentication
    }

    async fn sign(&self, data: Vec<u8>) -> Fallible<(MPublicKey, MSignature)> {
        let signature = self.private_key.sign(&data);
        Ok((self.private_key.public_key(), signature))
    }
}
