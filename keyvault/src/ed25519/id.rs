use blake2::{
    digest::{consts::U16, typenum::Unsigned},
    Blake2bMac,
};

use super::*;
use crate::multicipher::MKeyId;

type HashSize = U16;

/// This constant is used for keyed hashing of public keys. This does not improve the security
/// of the hash algorithm, but allows for domain separation if some use-case requires a different
/// hash of the public key with the same algorithm.
pub const KEY_ID_SALT: &[u8; 17] = b"open social graph";

/// The size of the key identifier in bytes. Since a version byte is prepended to the
/// hash result, it is not a standard size.
pub const KEY_ID_SIZE: usize = <HashSize as Unsigned>::USIZE + VERSION_SIZE;

/// The serialized byte representation for the current version of the hash algorithm
/// applied on the public key to obtain the key identifier
pub const KEY_ID_VERSION1: u8 = b'\x01';

/// Implementation of Ed25519::KeyId
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct EdKeyId(Vec<u8>);

impl EdKeyId {
    /// The key id serialized in a format that can be fed to [`from_bytes`]
    ///
    /// [`from_bytes`]: #method.from_bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }

    /// Creates a key id from a byte slice possibly returned by the [`to_bytes`] method.
    ///
    /// # Error
    /// If `bytes` is not [`KEY_ID_SIZE`] long
    ///
    /// [`to_bytes`]: #method.to_bytes
    /// [`KEY_ID_SIZE`]: ../constant.KEY_ID_SIZE
    pub fn from_bytes<D: AsRef<[u8]>>(bytes: D) -> Result<Self> {
        let bytes = bytes.as_ref();
        ensure!(bytes.len() == KEY_ID_SIZE, "Identifier length is not {}", KEY_ID_SIZE);
        ensure!(
            bytes[0] == KEY_ID_VERSION1,
            "Only identifier version {:x} is supported",
            KEY_ID_VERSION1
        );
        Ok(Self(bytes.to_owned()))
    }
}

type Blake2bMacVar = Blake2bMac<HashSize>;

impl From<&EdPublicKey> for EdKeyId {
    fn from(pk: &EdPublicKey) -> EdKeyId {
        let mut hasher = <Blake2bMacVar as KeyInit>::new_from_slice(KEY_ID_SALT)
            .expect("KEY_ID_SALT is shorter than 512 bits for Blake2b512; qed");
        hasher.update(&pk.to_bytes());
        let mut hash = [0u8; KEY_ID_SIZE];
        hash[0] = KEY_ID_VERSION1;
        hash[VERSION_SIZE..].clone_from_slice(hasher.finalize().into_bytes().as_slice());
        EdKeyId(hash.into())
    }
}

impl fmt::Debug for EdKeyId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let id = MKeyId::from(self.clone());
        id.fmt(formatter)
    }
}
