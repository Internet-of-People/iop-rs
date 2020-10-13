use iop_morpheus_core::{crypto::sign::SyncMorpheusSigner, data::auth::Authentication};

use super::*;

#[derive(Clone, Debug)]
pub struct Transaction {
    common_fields: CommonTransactionFields,
    asset: MorpheusAsset,
}

impl Transaction {
    pub fn new(
        common_fields: CommonTransactionFields, operation_attempts: Vec<OperationAttempt>,
    ) -> Self {
        Self { common_fields, asset: MorpheusAsset { operation_attempts } }
    }

    pub fn fee(&self) -> u64 {
        self.asset.fee()
    }
}

impl Aip29Transaction for Transaction {
    fn fee(&self) -> u64 {
        self.asset.fee()
    }

    fn to_data(&self) -> TransactionData {
        let mut tx_data: TransactionData = self.common_fields.to_data();
        tx_data.typed_asset = self.asset.to_owned().into();
        tx_data.fee = self.common_fields.calculate_fee(self).to_string();
        tx_data
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MorpheusAsset {
    pub operation_attempts: Vec<OperationAttempt>,
}

impl MorpheusAsset {
    const FEE_BYTES_OFFSET: u64 = 15;
    const FLAKES_PER_BYTES: u64 = 3000;

    pub fn fee(&self) -> u64 {
        let op_attempts_json = serde_json::to_string(&self.operation_attempts)
            .expect("Implementation error: serializing operation attempts must not fail");
        let bytes = (op_attempts_json.len() as u64).checked_add(Self::FEE_BYTES_OFFSET);
        bytes.and_then(|bytes| bytes.checked_mul(Self::FLAKES_PER_BYTES)).unwrap_or(u64::MAX)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let asset_json = serde_json::to_string(self)?;
        IopAsset::string_to_protobuf(&asset_json)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "operation")]
pub enum OperationAttempt {
    RegisterBeforeProof {
        #[serde(rename = "contentId")]
        content_id: String,
    },
    Signed(SignedOperation),
}

#[derive(Clone, Debug, Default)]
pub struct SignableOperation {
    signables: Vec<SignableOperationAttempt>,
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
        // NOTE this is a weird historical implementation detail with double-escaping,
        //      ideally should not be here, but fixing would require a hardfork
        let asset_str = serde_json::to_string(&asset_json)?;
        IopAsset::string_to_protobuf(&asset_str)
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
    signables: Vec<SignableOperationAttempt>,
    signer_public_key: String,
    signature: String,
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
