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

    pub fn add(mut self, attempt: SignableOperationAttempt) -> Self {
        self.signables.push(attempt);
        self
    }

    // TODO signing should use a dedicated sign_morpheus_transaction() operation,
    //      consider how this connects to that or this can be removed on the long run
    fn to_signable_bytes(&self) -> Result<Vec<u8>> {
        let asset_val = serde_json::to_value(&self.signables)?;
        let asset_json = canonical_json(&asset_val)?;
        Ok(serializer::frame_bytes(asset_json.as_bytes())?)
    }

    pub fn sign(self, signer: &dyn SyncMorpheusSigner) -> Result<SignedOperation> {
        let (signed_with_pubkey, signature) = signer.sign(&self.to_signable_bytes()?)?;
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
    pub signables: Vec<SignableOperationAttempt>,
    pub signer_public_key: String,
    pub signature: String,
}

// TODO Did probably should be strongly typed, but that complicates serialization as well.
//      Also consider using some stronger type for last_tx_id
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignableOperationAttempt {
    pub did: String,
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
