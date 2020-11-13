use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NoncedBundle {
    pub(crate) operations: Vec<UserOperation>,
    pub(crate) nonce: Nonce,
}

impl NoncedBundle {
    pub fn new(operations: Vec<UserOperation>, nonce: Nonce) -> Self {
        Self { operations, nonce }
    }

    pub fn sign(self, sk: &MPrivateKey) -> Result<SignedBundle> {
        let signature = sk.sign(self.serialize()?);
        let public_key = sk.public_key();
        Ok(SignedBundle { bundle: self, public_key, signature })
    }

    pub fn serialize(&self) -> Result<String> {
        let data = serde_json::to_value(&self)?;
        json_digest::canonical_json(&data)
    }
}

impl Priced for NoncedBundle {
    fn get_price(&self) -> Price {
        self.operations
            .iter()
            .try_fold(Price::zero(), |price, op| price.checked_add(op.get_price()))
            .unwrap_or(Price::fee(u64::MAX))
    }
}

// TODO think about reusing Signed<T> as in morpheus SignedJson and SignedBytes
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SignedBundle {
    #[serde(flatten)]
    pub(crate) bundle: NoncedBundle,
    #[serde(with = "serde_str")]
    pub(crate) public_key: MPublicKey,
    #[serde(with = "serde_str")]
    pub(crate) signature: MSignature,
}

impl SignedBundle {
    /// Verifies whether the operations are correctly signed with the public key provided
    pub fn verify(&self) -> bool {
        self.bundle.serialize().map_or(false, |s| self.public_key.verify(s, &self.signature))
    }
}

impl Priced for SignedBundle {
    fn get_price(&self) -> Price {
        self.bundle.get_price()
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

        let signed = NoncedBundle::new(vec![op1, op2], 42).sign(&sk).unwrap();

        assert!(signed.verify());
    }

    #[test]
    fn tampered_verify_fails() {
        let sk = ark_sk();
        let op1 = UserOperation::update(domain_name(), json! { "apfelstrudel" });
        let op2 = UserOperation::transfer(domain_name(), Principal::system());

        let mut signed = NoncedBundle::new(vec![op1, op2], 42).sign(&sk).unwrap();
        signed.bundle.operations[0] = UserOperation::update(domain_name(), json! { "braune soße" });

        assert!(!signed.verify());
    }

    #[test]
    fn serde_roundtrip() {
        let sk = ark_sk();
        let op1 = UserOperation::update(domain_name(), json! { "apfelstrudel" });
        let op2 = UserOperation::transfer(domain_name(), Principal::system());
        let signed = NoncedBundle::new(vec![op1, op2], 42).sign(&sk).unwrap();

        let serialized = serde_json::to_string(&signed).unwrap();

        // println!("{}", serde_json::to_string_pretty(&signed).unwrap());

        let deserialized: SignedBundle = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, signed);
    }
}
