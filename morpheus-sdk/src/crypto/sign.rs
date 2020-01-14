use async_trait::async_trait;
use failure::Fallible;
use serde::{Deserialize, Serialize};

use crate::crypto::hash::{Content, ContentId};
use crate::data::diddoc::Authentication;
use keyvault::{
    multicipher::{MPublicKey, MSignature},
    PublicKey,
};

#[async_trait(?Send)]
pub trait Signer {
    // TODO is it reasonable to assume that signing can always return the public key?
    async fn sign(&self, data: &[u8], auth: &Authentication) -> Fallible<(MPublicKey, MSignature)>;
}

pub type Nonce = u32;

#[async_trait(?Send)]
pub trait Signable: Content {
    fn content_to_sign(&self) -> Fallible<Vec<u8>> {
        let content_id = self.content_id()?;
        let content_id_bytes: &[u8] = (&content_id).into();
        Ok(content_id_bytes.to_owned())
    }

    async fn sign(&self, signer: &dyn Signer, auth: &Authentication) -> Fallible<Signed<Self>> {
        let (public_key, signature) = signer.sign(&self.content_to_sign()?, auth).await?;
        Ok(Signed { message: self.clone(), public_key, signature })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Signed<T>
where
    T: Signable,
{
    message: T,
    public_key: MPublicKey,
    signature: MSignature,
    // TODO is it OK here or should be given somewhere else?
    // TODO ClaimPresentation might be needed to prove proper right of delegated signing.
    // on_behalf_of: Did,
    // TODO consider adding a nonce here
}

impl<T> Signed<T>
where
    T: Signable,
{
    pub fn new(public_key: MPublicKey, message: T, signature: MSignature) -> Self {
        Self { public_key, message, signature }
    }

    pub fn message(&self) -> &T {
        &self.message
    }
    pub fn public_key(&self) -> &MPublicKey {
        &self.public_key
    }
    pub fn signature(&self) -> &MSignature {
        &self.signature
    }

    pub fn validate(&self) -> Fallible<bool> {
        let valid = self.public_key.verify(&self.message.content_to_sign()?, &self.signature);
        Ok(valid)
    }
}

pub type BlockHash = ContentId;

// TODO Eq, PartialEq and maybe PartialOrd for AfterEnvelope
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AfterEnvelope<T: Signable> {
    // TODO will contentId be fetched from the content or needs a separate field?
    //      should we just use the contentId here and provide another way to resolve the content from it?
    content: T,
    block: BlockHash,
    // TODO is a transactionId also needed here?
}

impl<T: Signable> Content for AfterEnvelope<T> {}
impl<T: Signable> Signable for AfterEnvelope<T> {}
