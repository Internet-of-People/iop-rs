use super::*;

/// The size of the private key in the format used by [`to_bytes`]
///
/// [`to_bytes`]: #method.to_bytes
pub const PRIVATE_KEY_SIZE: usize = ed::SECRET_KEY_LENGTH;

/// Implementation of Ed25519::PrivateKey
#[derive(Clone)]
pub struct EdPrivateKey(ed::SigningKey);

impl EdPrivateKey {
    /// The private key serialized in a format that can be fed to [`from_bytes`]
    ///
    /// [`from_bytes`]: #method.from_bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(PRIVATE_KEY_SIZE);
        res.extend_from_slice(self.0.as_bytes());
        res
    }

    /// Creates a public key from a byte slice possibly returned by the [`to_bytes`] method.
    ///
    /// # Error
    /// If `bytes` is rejected by `ed25519_dalek::SecretKey::from_bytes`
    ///
    /// [`to_bytes`]: #method.to_bytes
    pub fn from_bytes<D: AsRef<[u8]>>(bytes: D) -> Result<Self> {
        let secret_key = ed::SecretKey::try_from(bytes.as_ref())?;
        let signing_key = ed::SigningKey::from_bytes(&secret_key);
        Ok(Self(signing_key))
    }
}

impl PrivateKey<Ed25519> for EdPrivateKey {
    fn public_key(&self) -> EdPublicKey {
        let pk = self.0.verifying_key();
        pk.into()
    }
    fn sign<D: AsRef<[u8]>>(&self, data: D) -> EdSignature {
        let sig = self.0.sign(data.as_ref());
        sig.into()
    }
}

impl From<ed::SigningKey> for EdPrivateKey {
    fn from(sk: ed::SigningKey) -> Self {
        Self(sk)
    }
}

impl From<EdPrivateKey> for ed::SigningKey {
    fn from(sk: EdPrivateKey) -> ed::SigningKey {
        sk.0
    }
}
