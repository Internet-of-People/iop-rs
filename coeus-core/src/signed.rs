use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NoncedOperations {
    pub(crate) operations: Vec<UserOperation>,
    pub(crate) nonce: Nonce,
}

impl NoncedOperations {
    pub fn new(operations: Vec<UserOperation>, nonce: Nonce) -> Self {
        Self { operations, nonce }
    }

    pub fn sign(self, sk: &MPrivateKey) -> Result<SignedOperations> {
        let signature = sk.sign(self.serialize()?);
        let public_key = sk.public_key();
        Ok(SignedOperations { operations: self, public_key, signature })
    }

    pub fn serialize(&self) -> Result<String> {
        let data = serde_json::to_value(&self)?;
        json_digest::canonical_json(&data)
    }
}

impl Priced for NoncedOperations {
    fn get_price(&self, state: &State) -> Price {
        let mut price = Price::zero();
        for op in &self.operations {
            price += op.get_price(state);
        }
        price
    }
}

// TODO think about reusing Signed<T> as in morpheus SignedJson and SignedBytes
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SignedOperations {
    #[serde(flatten)]
    pub(crate) operations: NoncedOperations,
    #[serde(with = "serde_strz")]
    pub(crate) public_key: MPublicKey,
    #[serde(with = "serde_strz")]
    pub(crate) signature: MSignature,
}

impl SignedOperations {
    /// Verifies whether the operations are correctly signed with the public key provided
    pub fn verify(&self) -> bool {
        self.operations.serialize().map_or(false, |s| self.public_key.verify(s, &self.signature))
    }
}

impl Priced for SignedOperations {
    fn get_price(&self, state: &State) -> Price {
        self.operations.get_price(state)
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;

    pub(crate) fn ark_sk_from(passphrase: &str) -> MPrivateKey {
        let secp_sk =
            iop_keyvault::secp256k1::SecpPrivateKey::from_ark_passphrase(passphrase).unwrap();
        MPrivateKey::from(secp_sk)
    }

    pub fn ark_sk() -> MPrivateKey {
        let passphrase = "scout try doll stuff cake welcome random taste load town clerk ostrich";
        ark_sk_from(passphrase)
    }

    fn domain_name() -> DomainName {
        ".wallet.joe".parse().unwrap()
    }

    #[test]
    fn sign_and_verify() {
        let sk = ark_sk();
        let op1 = UserOperation::update(domain_name(), json! { "apfelstrudel" });
        let op2 = UserOperation::transfer(domain_name(), Principal::system());

        let signed = NoncedOperations::new(vec![op1, op2], 42).sign(&sk).unwrap();

        assert!(signed.verify());
    }

    #[test]
    fn tampered_verify_fails() {
        let sk = ark_sk();
        let op1 = UserOperation::update(domain_name(), json! { "apfelstrudel" });
        let op2 = UserOperation::transfer(domain_name(), Principal::system());

        let mut signed = NoncedOperations::new(vec![op1, op2], 42).sign(&sk).unwrap();
        signed.operations.operations[0] =
            UserOperation::update(domain_name(), json! { "braune soße" });

        assert!(!signed.verify());
    }

    #[test]
    fn serde_roundtrip() {
        let sk = ark_sk();
        let op1 = UserOperation::update(domain_name(), json! { "apfelstrudel" });
        let op2 = UserOperation::transfer(domain_name(), Principal::system());
        let signed = NoncedOperations::new(vec![op1, op2], 42).sign(&sk).unwrap();

        let serialized = serde_json::to_string(&signed).unwrap();

        // println!("{}", serde_json::to_string_pretty(&signed).unwrap());

        let deserialized: SignedOperations = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, signed);
    }
}
