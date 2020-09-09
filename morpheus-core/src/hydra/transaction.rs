use super::*;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TxBatch {
    pub transactions: Vec<TransactionData>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_group: Option<u32>,
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    //pub transaction_type: u32,
    // pub timestamp: u32, // present in the v2 schema only for v1 compatibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    pub sender_public_key: String,
    pub fee: String,
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<Asset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_signature: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub signatures: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<u64>,
}

impl TransactionData {
    pub fn set_type(&mut self, tx_type: TransactionType) {
        self.type_group = Some(tx_type.type_group());
        self.transaction_type = tx_type;
    }

    pub fn get_id(&self) -> Result<String> {
        let bytes = self.to_bytes(false, false, false)?;
        let id = hex::encode(Sha256::digest(&bytes));
        Ok(id)
    }

    pub fn to_bytes(
        &self, skip_signature: bool, skip_second_signature: bool, skip_multisignatures: bool,
    ) -> Result<Vec<u8>> {
        serializer::to_bytes(self, skip_signature, skip_second_signature, skip_multisignatures)
    }

    // pub fn second_sign(&mut self, passphrase: &str) -> Result<&mut Self> {
    //     let private_key = PrivateKey::from_passphrase(passphrase)?;
    //     let bytes = self.to_bytes(false, true, false)?;
    //     self.second_signature = Some(private_key.sign_ecdsa(&bytes)?);
    //     Ok(self)
    // }

    // pub fn verify(&self) -> bool {
    //     self.internal_verify(&self.sender_public_key, &self.signature, &self.to_bytes(true, true))
    // }
    //
    // pub fn second_verify(&self, sender_public_key: &str) -> bool {
    //     self.internal_verify(&sender_public_key, &self.sign_signature, &self.to_bytes(false, true))
    // }
    //
    // fn internal_verify(&self, sender_public_key: &str, signature: &str, bytes: &[u8]) -> bool {
    //     let hash = Sha256::digest(&bytes);
    //     let pk = PublicKey::from_hex(&sender_public_key).unwrap();
    //     let valid = pk.verify_signature_ecdsa(&hash, signature);
    //     valid.unwrap_or(false)
    // }
}
