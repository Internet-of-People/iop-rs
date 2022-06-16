use std::fmt;

use super::*;
use crate::multicipher::MSignature;

/// The serialized byte representation for the current version of the signature algorithm
pub const SIGNATURE_VERSION1: u8 = b'\x01';

/// Size of the signature is the version byte plus the actual libsecp256k1 signature size
pub const SIGNATURE_SIZE: usize = secp::util::SIGNATURE_SIZE + VERSION_SIZE;

/// Implementation of Secp256k1::Signature
#[derive(Clone, Eq, PartialEq)]
pub struct SecpSignature(pub(super) secp::Signature);

impl SecpSignature {
    /// The signature serialized in a format that can be fed to [`from_bytes`]
    ///
    /// [`from_bytes`]: #method.from_bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(SIGNATURE_SIZE);
        res.push(SIGNATURE_VERSION1);
        res.extend_from_slice(&self.0.serialize()[..]);
        res
    }

    /// Creates a signature from a byte slice possibly returned by the [`to_bytes`] method.
    ///
    /// # Error
    /// If `bytes` is rejected by `libsecp256k1::Signature::parse`
    ///
    /// [`to_bytes`]: #method.to_bytes
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self> {
        let bytes = bytes.as_ref();
        ensure!(bytes.len() == SIGNATURE_SIZE, "Signature length is not {}", SIGNATURE_SIZE);
        ensure!(
            bytes[0] == SIGNATURE_VERSION1,
            "Only identifier version {:x} is supported",
            SIGNATURE_VERSION1
        );
        let mut array = [0u8; secp::util::SIGNATURE_SIZE];
        array.copy_from_slice(&bytes[VERSION_SIZE..]);
        let sig = secp::Signature::parse_standard(&array)?;
        Ok(Self(sig))
    }

    /// The BTC mainnet uses a DER encoded signature, so many technology copied that over.
    ///
    /// This method allows to output such representation of
    pub fn to_der(&self) -> Vec<u8> {
        let array = self.0.serialize_der();
        array.as_ref().to_vec()
    }

    /// The BTC mainnet uses a DER encoded signature, so many technology copied that over.
    ///
    /// This method parses such signature into a SecpSignature. Very old BTC transactions
    /// also supported a less strict version (lax der), which we do not support.
    pub fn from_der(bytes: impl AsRef<[u8]>) -> Result<Self> {
        let sig = secp::Signature::parse_der(bytes.as_ref())?;
        Ok(Self(sig))
    }
}

impl fmt::Display for SecpSignature {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = MSignature::from(self.clone());
        id.fmt(formatter)
    }
}

impl fmt::Debug for SecpSignature {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}

impl From<secp::Signature> for SecpSignature {
    fn from(sig: secp::Signature) -> Self {
        Self(sig)
    }
}

impl From<SecpSignature> for secp::Signature {
    fn from(sig: SecpSignature) -> secp::Signature {
        sig.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn der_signature() {
        let passphrase = "scout try doll stuff cake welcome random taste load town clerk ostrich";
        let sk = SecpPrivateKey::from_ark_passphrase(passphrase).unwrap();

        let transfer_txn = "ff0280010000000000460000000000000003d4bda72219264ff106e21044b047b6c6b2c0dde8f49b42c848e086b97920adbf80969800000000000000e1f505000000000000000080954d93b02f3f0b189a9b308b15de1c4a550cf454";
        let sig_der_bytes = "3044022043e0c3379b364416d0eb154316e7c1ae9863afe4041c348efeb7c5b21b83c4610220451e3ef2f6c502356e2b962bf19632d690fe0e211268154bc34a7d7a57189739";

        let sig = sk.sign(&hex::decode(transfer_txn).unwrap());

        assert_eq!(hex::encode(sig.to_der()), sig_der_bytes);
    }
}
