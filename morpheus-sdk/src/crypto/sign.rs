use async_trait::async_trait;
use failure::Fallible;
use serde::{Deserialize, Serialize};

use crate::crypto::hash::{Content, ContentId};
use crate::data::auth::Authentication;
use crate::data::diddoc::{BlockHeight, DidDocument, Right};
use crate::data::serde_string;
use crate::data::validation::ValidationStatus;
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

// NOTE  multibase-encoded random content, e.g. 'urvU8F6HmEol5zOmHh_nnS1RiX5r3T2t9U_d_kQY7ZC-I"
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Nonce(pub String);

impl Nonce {
    pub fn new() -> Self {
        use rand::{thread_rng, RngCore};
        let mut arr = [0u8; 33];
        thread_rng().fill_bytes(&mut arr[..]);
        let encoded = multibase::encode(multibase::Base::Base64Url, &arr[..]);
        Self(encoded)
    }
}

#[async_trait(?Send)]
pub trait Signable: Content {
    fn content_to_sign(&self) -> Fallible<Vec<u8>> {
        let content_id = self.content_id()?;
        Ok(content_id.into_bytes())
    }

    async fn sign(&self, signer: &dyn Signer) -> Fallible<Signed<Self>> {
        let (public_key, signature) = signer.sign(self.content_to_sign()?).await?;
        Ok(Signed { content: self.clone(), public_key, signature, nonce: None })
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
#[serde(from = "SignatureSerializationFormat<T>", into = "SignatureSerializationFormat<T>")]
pub struct Signed<T>
where
    T: Signable,
{
    content: T,
    public_key: MPublicKey,
    signature: MSignature,
    nonce: Option<Nonce>,
    // TODO ClaimPresentation might be needed to prove proper right of delegated signing.
    // on_behalf_of: Did,
}

impl<T> Signed<T>
where
    T: Signable,
{
    pub fn new(public_key: MPublicKey, content: T, signature: MSignature) -> Self {
        Self { public_key, content, signature, nonce: None }
    }

    pub fn message(&self) -> &T {
        &self.content
    }
    pub fn public_key(&self) -> &MPublicKey {
        &self.public_key
    }
    pub fn signature(&self) -> &MSignature {
        &self.signature
    }

    pub fn validate(&self) -> Fallible<bool> {
        let valid = self.public_key.verify(&self.content.content_to_sign()?, &self.signature);
        Ok(valid)
    }

    pub fn validate_with_keyid(&self, signer_id: Option<MKeyId>) -> Fallible<bool> {
        let mut valid = self.validate()?;
        if let Some(id) = signer_id {
            valid &= self.public_key.validate_id(&id);
        }
        Ok(valid)
    }
}

// TODO probably this shouldn't be generic but work only with Before/AfterProofs
impl<T> Signed<T>
where
    T: Signable,
{
    // TODO add Before/AfterProofs as optional arguments here
    // TODO consider returning ValidationResult with issue vector and translate to status
    //      somewhere above in an upper layer
    pub fn validate_with_did_doc(&self, on_behalf_of: &DidDocument) -> Fallible<ValidationStatus> {
        if !self.validate()? {
            return Ok(ValidationStatus::Invalid);
        }

        let auth = Authentication::PublicKey(self.public_key.to_owned());
        let issues = on_behalf_of.validate_right_between(
            &auth,
            Right::Impersonation,
            1,
            on_behalf_of.queried_at_height,
        )?;
        Ok(issues.status())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct SignatureTuple {
    #[serde(with = "serde_string", rename = "publicKey")]
    public_key: MPublicKey,
    #[serde(with = "serde_string")]
    bytes: MSignature,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SignatureSerializationFormat<T> {
    signature: SignatureTuple,
    content: T,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    nonce: Option<Nonce>,
}

impl<T: Signable> From<Signed<T>> for SignatureSerializationFormat<T> {
    fn from(src: Signed<T>) -> Self {
        SignatureSerializationFormat {
            content: src.content,
            signature: SignatureTuple { public_key: src.public_key, bytes: src.signature },
            nonce: src.nonce,
        }
    }
}

impl<T: Signable> From<SignatureSerializationFormat<T>> for Signed<T> {
    fn from(src: SignatureSerializationFormat<T>) -> Self {
        Signed {
            content: src.content,
            public_key: src.signature.public_key,
            signature: src.signature.bytes,
            nonce: src.nonce,
        }
    }
}

pub type BlockHash = ContentId;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AfterProof {
    #[serde(rename = "blockHash")]
    block_hash: BlockHash,
    #[serde(rename = "blockHeight")]
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
