use serde::{Deserialize, Deserializer, Serialize, Serializer};

use keyvault::{multicipher, PublicKey};

const PREFIX_MULTICIPHER_KEYID: &str = "I"; // TODO this should be imported from the multicipher module
const PREFIX_DID_MORPHEUS: &str = "did:morpheus:";

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub struct Did {
    key_id: multicipher::MKeyId,
}

impl Serialize for Did {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        multicipher::MKeyId::serialize(&self.key_id, serializer)
    }
}

impl<'de> Deserialize<'de> for Did {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        multicipher::MKeyId::deserialize(deserializer).map(|key_id| Did::new(key_id))
    }
}

impl Did {
    pub fn new(key_id: multicipher::MKeyId) -> Self {
        Self { key_id }
    }

    pub fn key_id(&self) -> multicipher::MKeyId {
        self.key_id.to_owned()
    }
}

impl std::fmt::Display for Did {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", String::from(self))
    }
}

impl From<multicipher::MKeyId> for Did {
    fn from(src: multicipher::MKeyId) -> Self {
        Did::new(src)
    }
}

impl From<&multicipher::MKeyId> for Did {
    fn from(src: &multicipher::MKeyId) -> Self {
        src.to_owned().into()
    }
}

impl std::str::FromStr for Did {
    type Err = failure::Error;
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        if !src.starts_with(PREFIX_DID_MORPHEUS) {
            failure::bail!("{} is not a valid DID: must start with {}", src, PREFIX_DID_MORPHEUS);
        }
        let mkeyid = src.replacen(PREFIX_DID_MORPHEUS, PREFIX_MULTICIPHER_KEYID, 1);
        Ok(Did::new(mkeyid.parse()?))
    }
}

impl From<&Did> for String {
    fn from(src: &Did) -> Self {
        let key_id_str = src.key_id.to_string();
        // if !key_id_str.starts_with(PREFIX_MULTICIPHER_KEYID) {
        //     panic!("Implementation error: {} is not a valid KeyId: must start with {}", key_id_str, PREFIX_MULTICIPHER_KEYID );
        // }
        let result = key_id_str.replacen(PREFIX_MULTICIPHER_KEYID, PREFIX_DID_MORPHEUS, 1);
        result
    }
}

impl From<Did> for String {
    fn from(src: Did) -> Self {
        (&src).into()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SignedMessage {
    public_key: multicipher::MPublicKey,
    #[serde(with = "serde_bytes")]
    message: Vec<u8>,
    signature: multicipher::MSignature,
}

impl SignedMessage {
    pub fn new(
        public_key: multicipher::MPublicKey, message: Vec<u8>, signature: multicipher::MSignature,
    ) -> Self {
        Self { public_key, message, signature }
    }

    pub fn public_key(&self) -> &multicipher::MPublicKey {
        &self.public_key
    }
    pub fn message(&self) -> &[u8] {
        &self.message
    }
    pub fn signature(&self) -> &multicipher::MSignature {
        &self.signature
    }

    pub fn validate(&self) -> bool {
        self.public_key.verify(&self.message, &self.signature)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub struct DidDocument {}

impl DidDocument {}

#[cfg(test)]
mod tests {
    use super::*;
    use failure::Fallible;

    #[test]
    fn did_format() -> Fallible<()> {
        {
            let key_id = "IezbeWGSY2dqcUBqT8K7R14xr";
            let did = Did::new(key_id.parse()?);
            let did_str = did.to_string();
            assert_eq!(did_str, "did:morpheus:ezbeWGSY2dqcUBqT8K7R14xr");
            assert_eq!(did, did_str.parse()?);
        }
        {
            let key_id = "Iez25N5WZ1Q6TQpgpyYgiu9gTX";
            let did = Did::new(key_id.parse()?);
            let did_str = did.to_string();
            assert_eq!(did_str, "did:morpheus:ez25N5WZ1Q6TQpgpyYgiu9gTX");
            assert_eq!(did, did_str.parse()?);
        }
        Ok(())
    }
}
