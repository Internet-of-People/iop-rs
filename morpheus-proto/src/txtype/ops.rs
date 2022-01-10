use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "operation")]
pub enum OperationAttempt {
    RegisterBeforeProof {
        #[serde(rename = "contentId")]
        content_id: String,
    },
    Signed(SignedOperation),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SignableOperation {
    pub signables: Vec<SignableOperationAttempt>,
}

impl SignableOperation {
    pub fn new(signables: Vec<SignableOperationAttempt>) -> Self {
        Self { signables }
    }

    #[allow(clippy::should_implement_trait)]
    #[deprecated = "Use push instead"]
    pub fn add(self, attempt: SignableOperationAttempt) -> Self {
        self.push(attempt)
    }

    pub fn push(mut self, attempt: SignableOperationAttempt) -> Self {
        self.signables.push(attempt);
        self
    }

    // TODO signing should use a dedicated sign_morpheus_transaction() operation,
    //      consider how this connects to that or this can be removed on the long run
    pub fn to_signable_bytes(signables: &[SignableOperationAttempt]) -> Result<Vec<u8>> {
        let asset_val = serde_json::to_value(signables)?;
        let asset_json = canonical_json(&asset_val)?;
        serializer::frame_bytes(asset_json.as_bytes())
    }

    pub fn sign(self, signer: &dyn SyncMorpheusSigner) -> Result<SignedOperation> {
        let (signed_with_pubkey, signature) =
            signer.sign(&Self::to_signable_bytes(&self.signables)?)?;
        Ok(SignedOperation {
            signables: self.signables,
            signer_public_key: signed_with_pubkey.to_string(),
            signature: signature.to_string(),
        })
    }
}

// TDDO consider using strict types for public key and signature
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedOperation {
    signables: Vec<SignableOperationAttempt>,
    pub signer_public_key: String,
    pub signature: String,
}

impl SignedOperation {
    pub fn attempts_unsafe_without_signature_checking(
        &self,
    ) -> std::slice::Iter<'_, SignableOperationAttempt> {
        self.signables.iter()
    }

    pub fn attempts(&self) -> Result<std::slice::Iter<'_, SignableOperationAttempt>> {
        let bytes = SignableOperation::to_signable_bytes(&self.signables)?;
        let signer: MPublicKey = self.signer_public_key.parse()?;
        let signature: MSignature = self.signature.parse()?;
        ensure!(signer.verify(&bytes, &signature), "Invalid signature");
        Ok(self.attempts_unsafe_without_signature_checking())
    }
}

// TODO Did probably should be strongly typed, but that complicates serialization as well.
//      Also consider using some stronger type for last_tx_id
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignableOperationAttempt {
    #[serde(with = "serde_str")]
    pub did: Did,
    pub last_tx_id: Option<String>,
    #[serde(flatten)]
    pub operation: SignableOperationDetails,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "operation")]
pub enum SignableOperationDetails {
    AddKey {
        auth: Authentication,
        #[serde(rename = "expiresAtHeight", skip_serializing_if = "Option::is_none")]
        expires_at_height: Option<u32>,
    },
    RevokeKey {
        auth: Authentication,
    },
    AddRight {
        auth: Authentication,
        right: String,
    },
    RevokeRight {
        auth: Authentication,
        right: String,
    },
    TombstoneDid {},
}
