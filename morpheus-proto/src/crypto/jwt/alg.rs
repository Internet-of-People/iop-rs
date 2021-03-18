use std::borrow::Cow;

use anyhow::anyhow;

use super::*;

pub struct JwtSignature(MSignature);

impl AlgorithmSignature for JwtSignature {
    fn as_bytes(&self) -> std::borrow::Cow<'_, [u8]> {
        Cow::Owned(self.0.to_bytes())
    }

    fn try_from_slice(bytes: &[u8]) -> anyhow::Result<Self> {
        let inner = MSignature::from_bytes(bytes).map_err(|e| anyhow!("{}", e))?;
        Ok(JwtSignature(inner))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JwtMultiCipher;

impl Algorithm for JwtMultiCipher {
    type SigningKey = MPrivateKey;
    type VerifyingKey = MPublicKey;
    type Signature = JwtSignature;

    fn name(&self) -> Cow<'static, str> {
        Cow::Borrowed("Multicipher")
    }

    fn sign(&self, signing_key: &MPrivateKey, message: &[u8]) -> JwtSignature {
        JwtSignature(signing_key.sign(message))
    }

    fn verify_signature(
        &self, signature: &JwtSignature, verifying_key: &MPublicKey, message: &[u8],
    ) -> bool {
        verifying_key.verify(message, &signature.0)
    }
}
