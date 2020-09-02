use super::*;

#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[repr(u16)]
pub enum TransactionType {
    Transfer = 0,
    SecondSignatureRegistration = 1,
    DelegateRegistration = 2,
    Vote = 3,
    MultiSignatureRegistration = 4,
    Ipfs = 5,
    TimelockTransfer = 6,
    MultiPayment = 7,
    DelegateResignation = 8,
}

impl Default for TransactionType {
    fn default() -> Self {
        Self::Transfer
    }
}

impl TransactionType {
    pub const TYPE_GROUP: u32 = 1;

    pub fn fee(self) -> u64 {
        match self {
            Self::Transfer => 10_000_000,
            Self::SecondSignatureRegistration => 500_000_000,
            Self::DelegateRegistration => 2_500_000_000,
            Self::Vote => 100_000_000,
            Self::MultiSignatureRegistration => 500_000_000,
            Self::Ipfs => 0,
            Self::TimelockTransfer => 0,
            Self::MultiPayment => 0,
            Self::DelegateResignation => 0,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Asset {
    None,
    Signature {
        #[serde(rename = "publicKey")]
        public_key: String,
    },
    Delegate {
        username: String,
    },
    Votes(Vec<String>),
    #[serde(rename = "multiSignature")]
    MultiSignatureRegistration {
        #[serde(rename = "publicKeys")]
        public_keys: Vec<String>,
        min: u8,
    },
    Ipfs(String),
    Payments(Vec<PaymentsItem>),
    Lock {
        #[serde(rename = "secretHash")]
        secret_hash: String,
        expiration: LockExpiration,
    },
    Claim {
        #[serde(rename = "lockTransactionId")]
        lock_transaction_id: String,
        #[serde(rename = "unlockSecret")]
        unlock_secret: String,
    },
    #[serde(rename = "refund")]
    Refund {
        #[serde(rename = "lockTransactionId")]
        lock_transaction_id: String,
    },
    #[serde(rename = "businessRegistration")]
    BusinessRegistration {
        name: String,
        website: String,
    },
    #[serde(rename = "businessUpdate")]
    BusinessUpdate {
        name: String,
        website: String,
    },
    #[serde(rename = "bridgechainRegistration")]
    BridgeChainRegistration {
        name: String,
        #[serde(rename = "seedNodes")]
        seed_nodes: Vec<String>,
        #[serde(rename = "genesisHash")]
        genesis_hash: String,
        #[serde(rename = "bridgechainRepository")]
        bridgechain_repository: String,
        ports: HashMap<String, u32>,
    },
    #[serde(rename = "bridgechainUpdate")]
    BridgechainUpdate {
        #[serde(rename = "bridgechainId")]
        bridgechain_id: String,
        #[serde(rename = "seedNodes")]
        seed_nodes: Vec<String>,
        ports: HashMap<String, u32>,
    },
    #[serde(rename = "bridgechainResignation")]
    BridgechainResignation {
        #[serde(rename = "bridgechainId")]
        bridgechain_id: String,
    },
}

#[derive(Default, Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LockExpiration {
    #[serde(rename = "type")]
    pub expiration_type: u64,
    pub value: u64,
}

#[derive(Default, Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentsItem {
    pub amount: String,
    pub recipient_id: String,
}

impl Asset {
    pub fn is_none(&self) -> bool {
        match *self {
            Asset::None => true,
            _ => false,
        }
    }
}

impl Default for Asset {
    fn default() -> Self {
        Asset::None
    }
}

#[derive(Clone, Debug)]
pub struct Transaction {
    common_fields: CommonTransactionFields,
    tx_type: TransactionType,
    asset: Asset,
    recipient_id: Option<String>,
}

impl Transaction {
    pub fn new_transfer(common_fields: CommonTransactionFields, recipient_id: String) -> Self {
        Self {
            common_fields,
            tx_type: TransactionType::Transfer,
            recipient_id: Some(recipient_id),
            asset: Asset::None,
        }
    }

    pub fn new_delegate_registration(
        common_fields: CommonTransactionFields, username: &str,
    ) -> Self {
        Self {
            common_fields,
            tx_type: TransactionType::DelegateRegistration,
            recipient_id: None,
            asset: Asset::Delegate { username: username.to_owned() },
        }
    }

    // TODO consider having a SecpPublicKey parameter, adding a "+" prefix automatically
    //      and separating vote and unvote (adds "-" prefix) operations
    pub fn new_vote(common_fields: CommonTransactionFields, vote: &str) -> Self {
        Self {
            common_fields,
            tx_type: TransactionType::Vote,
            recipient_id: None,
            asset: Asset::Votes(vec![vote.to_owned()]),
        }
    }
}

impl Aip29Transaction for Transaction {
    fn fee(&self) -> u64 {
        self.tx_type.fee()
    }

    fn to_data(&self) -> TransactionData {
        let mut tx_data: TransactionData = self.common_fields.to_data();
        tx_data.set_type(crate::hydra::txtype::TransactionType::Core(self.tx_type));
        tx_data.asset = Some(crate::hydra::txtype::Asset::Core(self.asset.to_owned()));
        tx_data.recipient_id = self.recipient_id.to_owned();
        tx_data.fee = self.common_fields.calculate_fee(self).to_string();
        tx_data
    }
}
