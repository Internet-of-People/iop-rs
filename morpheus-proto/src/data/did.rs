use super::*;

// NOTE should be const, but current language rules do not allow that
fn prefix_multicipher_keyid() -> String {
    multicipher::MKeyId::PREFIX.to_string()
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub struct Did {
    default_key_id: multicipher::MKeyId,
}

impl Serialize for Did {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        multicipher::MKeyId::serialize(&self.default_key_id, serializer)
    }
}

impl<'de> Deserialize<'de> for Did {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        multicipher::MKeyId::deserialize(deserializer).map(Did::new)
    }
}

impl Did {
    pub const PREFIX: &'static str = "did:morpheus:";

    pub fn new(key_id: multicipher::MKeyId) -> Self {
        Self { default_key_id: key_id }
    }

    pub fn default_key_id(&self) -> multicipher::MKeyId {
        self.default_key_id.to_owned()
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

impl FromStr for Did {
    type Err = anyhow::Error;
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        if !src.starts_with(Self::PREFIX) {
            bail!("{} is not a valid DID: must start with {}", src, Self::PREFIX);
        }
        let mkeyid = src.replacen(Self::PREFIX, &prefix_multicipher_keyid(), 1);
        Ok(Did::new(mkeyid.parse()?))
    }
}

impl From<&Did> for String {
    fn from(src: &Did) -> Self {
        let key_id_str = src.default_key_id.to_string();
        // if !key_id_str.starts_with(prefix_multicipher_keyid) {
        //     panic!("Implementation error: {} is not a valid KeyId: must start with {}", key_id_str, prefix_multicipher_keyid );
        // }
        key_id_str.replacen(&prefix_multicipher_keyid(), Did::PREFIX, 1)
    }
}

impl From<Did> for String {
    fn from(src: Did) -> Self {
        (&src).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_did_id(did_str: &str, key_id_str: &str) -> Result<()> {
        let did = Did::new(key_id_str.parse()?);
        assert_eq!(did.to_string(), did_str);
        assert_eq!(did, did_str.parse()?);
        Ok(())
    }

    #[test]
    fn did_format() -> Result<()> {
        test_did_id("did:morpheus:ezbeWGSY2dqcUBqT8K7R14xr", "iezbeWGSY2dqcUBqT8K7R14xr")?;
        test_did_id("did:morpheus:ez25N5WZ1Q6TQpgpyYgiu9gTX", "iez25N5WZ1Q6TQpgpyYgiu9gTX")
    }
}
