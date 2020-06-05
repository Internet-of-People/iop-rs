use crate::*;
use std::fmt;

pub struct TestCrypto {}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct TestKeyId(String);

impl fmt::Debug for TestKeyId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestSignature {
    data: Vec<u8>,
    pub_key: TestPublicKey,
}

#[derive(Clone, Eq, PartialEq)]
pub struct TestPrivateKey(String);

impl fmt::Debug for TestPrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("sk({:})", self.0))
    }
}

impl PrivateKey<TestCrypto> for TestPrivateKey {
    fn public_key(&self) -> TestPublicKey {
        TestPublicKey(self.0.clone())
    }
    fn sign<D: AsRef<[u8]>>(&self, data: D) -> TestSignature {
        TestSignature { data: data.as_ref().to_owned(), pub_key: self.public_key() }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct TestPublicKey(String);

impl fmt::Debug for TestPublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("pk({:})", self.0))
    }
}

impl PublicKey<TestCrypto> for TestPublicKey {
    fn key_id(&self) -> TestKeyId {
        TestKeyId(format!("id({0})", self.0))
    }
    fn validate_id(&self, key_id: &TestKeyId) -> bool {
        &self.key_id() == key_id
    }
    fn verify<D: AsRef<[u8]>>(&self, data: D, sig: &TestSignature) -> bool {
        sig.data.as_slice() == data.as_ref() && *self == sig.pub_key
    }
}

impl AsymmetricCrypto for TestCrypto {
    type KeyId = TestKeyId;
    type PrivateKey = TestPrivateKey;
    type PublicKey = TestPublicKey;
    type Signature = TestSignature;
}

#[derive(Clone, Eq, PartialEq)]
pub struct TestXprv(TestPrivateKey);

impl fmt::Debug for TestXprv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("xprv({:?})", self.0))
    }
}

impl ExtendedPrivateKey<TestCrypto> for TestXprv {
    fn derive_normal_child(&self, idx: i32) -> Fallible<TestXprv> {
        Ok(TestXprv(TestPrivateKey(format!("{}/{}", (self.0).0, idx))))
    }
    fn derive_hardened_child(&self, idx: i32) -> Fallible<TestXprv> {
        Ok(TestXprv(TestPrivateKey(format!("{}/{}'", (self.0).0, idx))))
    }
    fn neuter(&self) -> TestXpub {
        TestXpub(TestPublicKey((self.0).0.clone()))
    }
    fn private_key(&self) -> TestPrivateKey {
        self.0.clone()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct TestXpub(TestPublicKey);

impl fmt::Debug for TestXpub {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("xpub({:?})", self.0))
    }
}

impl ExtendedPublicKey<TestCrypto> for TestXpub {
    fn derive_normal_child(&self, idx: i32) -> Fallible<TestXpub> {
        Ok(TestXpub(TestPublicKey(format!("{}/{}", (self.0).0, idx))))
    }
    fn public_key(&self) -> TestPublicKey {
        self.0.clone()
    }
}

impl KeyDerivationCrypto for TestCrypto {
    type ExtendedPrivateKey = TestXprv;
    type ExtendedPublicKey = TestXpub;

    fn master(_seed: &Seed) -> TestXprv {
        TestXprv(TestPrivateKey("m".to_owned()))
    }
}

pub struct TestNetwork;

impl Subtree for TestNetwork {
    type Suite = TestCrypto;
    fn name(&self) -> &'static str {
        "Test network"
    }
    fn master(&self, seed: &Seed) -> TestXprv {
        TestCrypto::master(seed)
    }
    fn key_id(&self, pk: &TestPublicKey) -> TestKeyId {
        pk.key_id()
    }
}

impl Network for TestNetwork {
    fn p2pkh_addr(&self) -> &'static [u8; ADDR_PREFIX_SIZE] {
        &[0]
    }
    fn p2sh_addr(&self) -> &'static [u8; ADDR_PREFIX_SIZE] {
        &[0]
    }
    fn wif(&self) -> &'static [u8; ADDR_PREFIX_SIZE] {
        &[0]
    }
    fn bip32_xprv(&self) -> &'static [u8; BIP32_VERSION_PREFIX_SIZE] {
        &[0, 0, 0, 0]
    }
    fn bip32_xpub(&self) -> &'static [u8; BIP32_VERSION_PREFIX_SIZE] {
        &[0, 0, 0, 0]
    }
    fn message_prefix(&self) -> &'static str {
        "Test message\n"
    }
    fn slip44(&self) -> i32 {
        42
    }
    fn subtree(&self) -> &dyn Subtree<Suite = TestCrypto> {
        self
    }
}
