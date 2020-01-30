use std::error::Error;

use failure::Fallible;
use multihash::Multihash;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Eq, Hash, PartialEq)]
pub struct ContentId {
    inner: Multihash,
}

impl ContentId {
    pub fn new(hash: Multihash) -> Self {
        Self { inner: hash }
    }

    pub fn hash(&self) -> &Multihash {
        &self.inner
    }
}

impl Serialize for ContentId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(self.inner.as_bytes())
        //serializer.serialize_str(self.inner.as_bytes().encode_as_some_string() )
    }
}

impl<'de> Deserialize<'de> for ContentId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = <Vec<u8>>::deserialize(deserializer)?;
        let inner = Multihash::from_bytes(bytes).map_err(|e| {
            serde::de::Error::custom(format!("Invalid multihash {}", e.description()))
        })?;
        Ok(Self::new(inner))
    }
}

impl Clone for ContentId {
    fn clone(&self) -> Self {
        let inner_bytes = self.inner.as_bytes().to_owned();
        let inner = Multihash::from_bytes(inner_bytes).expect(
            "Implementation error: multihash library cannot parse its own serialized format",
        );
        Self { inner }
    }
}

impl<'a> From<&'a ContentId> for &'a [u8] {
    fn from(src: &'a ContentId) -> Self {
        src.inner.as_bytes()
    }
}

impl From<&ContentId> for String {
    fn from(src: &ContentId) -> Self {
        multibase::encode(multibase::Base::Base58btc, src.inner.as_bytes())
    }
}

impl From<ContentId> for String {
    fn from(src: ContentId) -> Self {
        (&src).into()
    }
}

impl std::fmt::Display for ContentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "c{}", String::from(self))
    }
}

impl std::fmt::Debug for ContentId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        (self as &dyn std::fmt::Display).fmt(f)
    }
}

impl std::str::FromStr for ContentId {
    type Err = failure::Error;
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let (_base, hash_bytes) = multibase::decode(src)?;
        let hash = Multihash::from_bytes(hash_bytes)?;
        Ok(Self::new(hash))
    }
}

pub trait Content: Serialize + Clone + Sized {
    fn to_bytes(&self) -> Fallible<Vec<u8>> {
        // TODO consider a usable default
        Ok(serde_json::to_vec_pretty(self)?)
        // todo!()
    }

    fn content_id(&self) -> Fallible<ContentId> {
        let bytes = self.to_bytes()?;
        let hash = multihash::encode(multihash::Hash::Keccak256, &bytes)?;
        Ok(ContentId::new(hash))
    }

    fn validate_id(&self, content_id: &ContentId) -> Fallible<bool> {
        let bytes = self.to_bytes()?;
        let calculated_hash = multihash::encode(content_id.inner.algorithm(), &bytes)?;
        Ok(calculated_hash == content_id.inner)
    }
}

impl Content for &[u8] {}
impl Content for Vec<u8> {}
impl Content for &str {}
impl Content for String {}
