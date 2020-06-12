use std::fmt;
use std::ops::Add;
use std::str::FromStr;

use super::*;
use crate::PublicKey;

/// The size of the public key in the compressed format used by [`to_bytes`]
///
/// [`to_bytes`]: #method.to_bytes
pub const PUBLIC_KEY_SIZE: usize = secp::util::COMPRESSED_PUBLIC_KEY_SIZE;

/// The size of the public key in the uncompressed format used by [`uncompressed`]
///
/// [`uncompressed`]: #method.uncompressed
pub const PUBLIC_KEY_UNCOMPRESSED_SIZE: usize = secp::util::FULL_PUBLIC_KEY_SIZE;

/// Implementation of Secp256k1::PublicKey
#[derive(Clone, Eq, PartialEq)]
pub struct SecpPublicKey(pub(super) secp::PublicKey);

impl SecpPublicKey {
    /// The public key serialized in a format that can be fed to [`from_bytes`]
    ///
    /// [`from_bytes`]: #method.from_bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.serialize_compressed().to_vec()
    }

    /// Creates a public key from a byte slice possibly returned by the [`to_bytes`] method.
    ///
    /// # Error
    /// If `bytes` is rejected by `libsecp256k1::PublicKey::parse_slice`
    ///
    /// [`to_bytes`]: #method.to_bytes
    pub fn from_bytes<D: AsRef<[u8]>>(bytes: D) -> Fallible<Self> {
        let format = Some(secp::PublicKeyFormat::Compressed);
        let pk = secp::PublicKey::parse_slice(bytes.as_ref(), format).map_err(SecpError::from)?;
        Ok(Self(pk))
    }

    /// The public key serialized in the uncompressed format used in some places in the bitcoin
    /// ecosystem (like address hashing in [`SecpKeyId::bitcoin_address`])
    ///
    /// [`SecpKeyId::bitcoin_address`]: ../struct.SecpKeyId.html#method.bitcoin_address
    pub fn uncompressed(&self) -> [u8; PUBLIC_KEY_UNCOMPRESSED_SIZE] {
        self.0.serialize()
    }

    /// ARK uses a non-standards hashing of the compressed public key.
    pub fn ark_key_id(&self) -> SecpKeyId {
        SecpKeyId::from_ark_pk(self)
    }
}

impl Add<&[u8]> for &SecpPublicKey {
    type Output = Fallible<SecpPublicKey>;

    fn add(self, rhs: &[u8]) -> Self::Output {
        let sk = secp::SecretKey::parse_slice(rhs).map_err(SecpError::from)?;
        let mut sum = self.0.clone();
        sum.tweak_add_assign(&sk).map_err(SecpError::from)?;
        Ok(SecpPublicKey(sum))
    }
}

impl PublicKey<Secp256k1> for SecpPublicKey {
    fn key_id(&self) -> SecpKeyId {
        SecpKeyId::from(self)
    }

    fn validate_id(&self, key_id: &SecpKeyId) -> bool {
        &self.key_id() == key_id
    }

    fn verify<D: AsRef<[u8]>>(&self, data: D, sig: &SecpSignature) -> bool {
        let msg = Secp256k1::hash_message(data);
        secp::verify(&msg, &sig.0, &self.0)
    }
}

impl FromStr for SecpPublicKey {
    type Err = failure::Error;
    fn from_str(src: &str) -> Fallible<Self> {
        Self::from_bytes(hex::decode(src)?)
    }
}

impl fmt::Display for SecpPublicKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        formatter.write_str(&hex::encode(self.to_bytes()))
    }
}

impl fmt::Debug for SecpPublicKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self, formatter)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string_roundtrip() -> Fallible<()> {
        let key = "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
        let pk: SecpPublicKey = key.parse()?;
        assert_eq!(pk.to_string(), key);
        Ok(())
    }
}
