use std::ops::Add;

use super::*;
use crate::PrivateKey;

/// The size of the private key in the format used by [`to_bytes`]
///
/// [`to_bytes`]: #method.to_bytes
pub const PRIVATE_KEY_SIZE: usize = secp::util::SECRET_KEY_SIZE;

/// Implementation of Secp256k1::PrivateKey
#[derive(Clone, Eq, PartialEq)]
pub struct SecpPrivateKey(secp::SecretKey);

impl SecpPrivateKey {
    /// The private key serialized in a format that can be fed to [`from_bytes`]
    ///
    /// [`from_bytes`]: #method.from_bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.serialize().to_vec()
    }

    /// Creates a public key from a byte slice possibly returned by the [`to_bytes`] method.
    ///
    /// # Error
    /// If `bytes` is rejected by `libsecp256k1::SecretKey::parse_slice`
    ///
    /// [`to_bytes`]: #method.to_bytes
    pub fn from_bytes<D: AsRef<[u8]>>(bytes: D) -> Result<Self> {
        let sk = secp::SecretKey::parse_slice(bytes.as_ref())?;
        Ok(Self(sk))
    }

    /// Most ARK wallets simply hash a passphrase into a private key.
    pub fn from_ark_passphrase(phrase: impl AsRef<str>) -> Result<Self> {
        let hash = sha2::Sha256::digest(phrase.as_ref().as_bytes());
        Self::from_bytes(hash)
    }

    /// Serializes private key into wallet import format supported by many pre-HD wallets
    pub fn to_wif(&self, version: &[u8; ADDR_PREFIX_SIZE], usage: Bip178) -> String {
        let mut res = Vec::with_capacity(1 + 1 + PRIVATE_KEY_SIZE);
        res.extend_from_slice(version);
        res.extend_from_slice(&self.to_bytes());
        res.extend_from_slice(usage.to_wif_suffix());

        to_base58check(res)
    }

    /// Deserializes private key from wallet import format supported by many pre-HD wallets
    pub fn from_wif(wif: &str, network: &dyn Network<Suite = Secp256k1>) -> Result<(Self, Bip178)> {
        let data = from_base58check(wif)?;
        ensure!(data.len() > PRIVATE_KEY_SIZE, "WIF data is too short");

        let expected_prefix = network.wif();
        debug_assert_eq!(expected_prefix.len(), ADDR_PREFIX_SIZE);
        debug_assert_eq!(ADDR_PREFIX_SIZE, 1);

        let (actual_prefix, data) = data.split_at(ADDR_PREFIX_SIZE);
        ensure!(
            actual_prefix == expected_prefix,
            "Invalid network prefix found: {}",
            hex::encode(actual_prefix)
        );

        let (sk_bytes, usage_bytes) = data.split_at(PRIVATE_KEY_SIZE);
        let sk = Self::from_bytes(sk_bytes)?;
        let usage = Bip178::from_wif_suffix(usage_bytes)?;

        Ok((sk, usage))
    }
}

impl Add<&[u8]> for &SecpPrivateKey {
    type Output = Result<SecpPrivateKey>;

    fn add(self, rhs: &[u8]) -> Self::Output {
        let mut sum = secp::SecretKey::parse_slice(rhs)?;
        sum.tweak_add_assign(&self.0)?;
        Ok(SecpPrivateKey(sum))
    }
}

impl PrivateKey<Secp256k1> for SecpPrivateKey {
    fn public_key(&self) -> SecpPublicKey {
        let pk = secp::PublicKey::from_secret_key(&self.0);
        SecpPublicKey(pk)
    }

    /// # Panics
    /// There is a 2^-256 chance this message cannot be signed
    /// by this key. The C implementation in bitcoin does not
    /// fail, but this pure rust version does. Then we panic.
    fn sign<D: AsRef<[u8]>>(&self, data: D) -> SecpSignature {
        let msg = Secp256k1::hash_message(data);
        let (sig, _recovery) = secp::sign(&msg, &self.0);
        SecpSignature(sig)
    }
}

impl From<secp::SecretKey> for SecpPrivateKey {
    fn from(sk: secp::SecretKey) -> Self {
        Self(sk)
    }
}

impl From<SecpPrivateKey> for secp::SecretKey {
    fn from(sk: SecpPrivateKey) -> secp::SecretKey {
        sk.0
    }
}
