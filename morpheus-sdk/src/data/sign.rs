use failure::Fallible;
use serde::{Deserialize, Serialize};

use crate::data::hash::{Content, ContentId};
use keyvault::{multicipher, PublicKey};

pub type Nonce = u32;

pub trait Signable: Content {
    // async? fn sign_with_key(key: PrivateKey/Signer?) -> Fallible<SignedMessage>;
    // async? fn sign_with_key_id(keyId: KeyId/Signer?) -> Fallible<SignedMessage>;
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Signed<T>
where
    T: Signable,
{
    public_key: multicipher::MPublicKey,
    signature: multicipher::MSignature,
    message: T,
    // TODO is it OK here or should be given somewhere else?
    // TODO ClaimPresentation might be needed to prove proper right of delegated signing.
    // on_behalf_of: Did,
}

impl<T> Signed<T>
where
    T: Signable,
{
    pub fn new(
        public_key: multicipher::MPublicKey, message: T, signature: multicipher::MSignature,
    ) -> Self {
        Self { public_key, message, signature }
    }

    pub fn public_key(&self) -> &multicipher::MPublicKey {
        &self.public_key
    }
    pub fn message(&self) -> &T {
        &self.message
    }
    pub fn signature(&self) -> &multicipher::MSignature {
        &self.signature
    }

    pub fn validate(&self) -> Fallible<bool> {
        // TODO consider if we should sign the whole content or just its hash
        let valid = self.public_key.verify(&self.message.to_bytes()?, &self.signature);
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
