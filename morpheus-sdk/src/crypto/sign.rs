use async_trait::async_trait;
use failure::Fallible;
use serde::{Deserialize, Serialize};

use crate::crypto::hash::{Content, ContentId};
use crate::data::auth::Authentication;
use crate::data::diddoc::{BlockHeight, DidDocument, Right};
use crate::data::serde_string;
use crate::io::dist::did::ValidationStatus;
use keyvault::{
    multicipher::{MKeyId, MPrivateKey, MPublicKey, MSignature},
    PrivateKey, PublicKey,
};

// TODO signer trait maybe belongs more under crate::io::local
#[async_trait(?Send)]
pub trait Signer {
    fn authentication(&self) -> &Authentication;

    // TODO MUST BE CHANGED to have separated special-purpose signer functions
    //      so that the user can receive a confirmation dialog with relevant semantics
    //      fn sign(&self, req: &WitnessRequest) -> Fallible<Signed<WitnessRequest>>
    async fn sign(&self, data: Vec<u8>) -> Fallible<(MPublicKey, MSignature)>;
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

pub type Nonce = u32;

#[async_trait(?Send)]
pub trait Signable: Content {
    fn content_to_sign(&self) -> Fallible<Vec<u8>> {
        let content_id = self.content_id()?;
        let content_id_bytes: &[u8] = (&content_id).into();
        Ok(content_id_bytes.to_owned())
    }

    async fn sign(&self, signer: &dyn Signer) -> Fallible<Signed<Self>> {
        let (public_key, signature) = signer.sign(self.content_to_sign()?).await?;
        Ok(Signed { message: self.clone(), public_key, signature })
    }
}

impl Signable for &[u8] {
    fn content_to_sign(&self) -> Fallible<Vec<u8>> {
        Ok((*self).to_owned())
    }
}
impl Signable for Vec<u8> {
    fn content_to_sign(&self) -> Fallible<Vec<u8>> {
        Ok(self.clone())
    }
}
impl Signable for &str {
    fn content_to_sign(&self) -> Fallible<Vec<u8>> {
        Ok(self.as_bytes().to_owned())
    }
}
impl Signable for String {
    fn content_to_sign(&self) -> Fallible<Vec<u8>> {
        Ok(self.as_bytes().to_owned())
    }
}

// TODO implement Hash for MPublicKey and MSignature
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Signed<T>
where
    T: Signable,
{
    message: T,
    #[serde(with = "serde_string")]
    public_key: MPublicKey,
    #[serde(with = "serde_string")]
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

    pub fn validate_with_keyid(&self, signer_id: Option<MKeyId>) -> Fallible<bool> {
        let mut valid = self.validate()?;
        if let Some(id) = signer_id {
            valid &= self.public_key.validate_id(&id);
        }
        Ok(valid)
    }

    pub fn validate_with_did(
        &self, on_behalf_of: &DidDocument, signer_id: Option<MKeyId>,
    ) -> Fallible<ValidationStatus> {
        let auth = match signer_id.as_ref() {
            Some(id) => Authentication::KeyId(id.to_owned()),
            None => Authentication::PublicKey(self.public_key.to_owned()),
        };
        if !self.validate_with_keyid(signer_id)? {
            return Ok(ValidationStatus::Invalid);
        }
        Ok(on_behalf_of.has_right(&auth, Right::Impersonation, 1, on_behalf_of.at_height))
    }
}

pub type BlockHash = ContentId;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AfterProof {
    block_hash: BlockHash,
    block_height: BlockHeight,
}

//impl Content for AfterProof {}
//impl Signable for AfterProof {}

// TODO Eq, PartialEq and maybe PartialOrd for AfterEnvelope
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AfterEnvelope<T: Signable> {
    // TODO will contentId be fetched from the content or needs a separate field?
    //      should we just use the contentId here and provide another way to resolve the content from it?
    content: T,
    proof: AfterProof, // TODO is a transactionId also needed here?
}

impl<T: Signable> Content for AfterEnvelope<T> {}
impl<T: Signable> Signable for AfterEnvelope<T> {}
