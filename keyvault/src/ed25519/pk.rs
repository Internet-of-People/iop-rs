use super::*;

/// The size of the public key in the compressed format used by [`to_bytes`]
///
/// [`to_bytes`]: #method.to_bytes
pub const PUBLIC_KEY_SIZE: usize = ed::PUBLIC_KEY_LENGTH;

/// Implementation of Ed25519::PublicKey
#[derive(Clone, Eq, PartialEq)]
pub struct EdPublicKey(ed::PublicKey);

impl EdPublicKey {
    /// The public key serialized in a format that can be fed to [`from_bytes`]
    ///
    /// [`from_bytes`]: #method.from_bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(PUBLIC_KEY_SIZE);
        res.extend_from_slice(self.0.as_bytes());
        res
    }

    /// Creates a public key from a byte slice possibly returned by the [`to_bytes`] method.
    ///
    /// # Error
    /// If `bytes` is rejected by `ed25519_dalek::PublicKey::from_bytes`
    ///
    /// [`to_bytes`]: #method.to_bytes
    pub fn from_bytes<D: AsRef<[u8]>>(bytes: D) -> Result<Self> {
        let pk = ed::PublicKey::from_bytes(bytes.as_ref())?;
        Ok(Self(pk))
    }
}

impl From<ed::PublicKey> for EdPublicKey {
    fn from(pk: ed::PublicKey) -> Self {
        Self(pk)
    }
}

impl<'a> From<&'a EdPublicKey> for &'a ed::PublicKey {
    fn from(pk: &'a EdPublicKey) -> &'a ed::PublicKey {
        &pk.0
    }
}

impl PublicKey<Ed25519> for EdPublicKey {
    fn key_id(&self) -> EdKeyId {
        EdKeyId::from(self)
    }
    fn validate_id(&self, key_id: &EdKeyId) -> bool {
        &self.key_id() == key_id
    }
    /// We should never assume that there is only 1 public key that can verify a given
    /// signature. Actually, there are 8 public keys.
    fn verify<D: AsRef<[u8]>>(&self, data: D, sig: &EdSignature) -> bool {
        let res = self.0.verify(data.as_ref(), sig.into());
        res.is_ok()
    }
}

#[allow(clippy::derive_hash_xor_eq)] // If the 2 pks are equal. their hashes will be equal, too
impl Hash for EdPublicKey {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.to_bytes().hash(hasher);
    }
}

impl PartialOrd<Self> for EdPublicKey {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for EdPublicKey {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.to_bytes().cmp(&rhs.to_bytes())
    }
}

impl ExtendedPublicKey<Ed25519> for EdPublicKey {
    fn derive_normal_child(&self, _idx: i32) -> Result<EdPublicKey> {
        bail!("Normal derivation of Ed25519 is invalid based on SLIP-0010.")
    }
    fn public_key(&self) -> EdPublicKey {
        self.clone()
    }
}
