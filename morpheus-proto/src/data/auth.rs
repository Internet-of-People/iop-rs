use super::*;

use iop_keyvault::{multicipher, PublicKey};

#[derive(Clone, Debug, Deserialize, Eq, Serialize)]
#[serde(untagged)]
pub enum Authentication {
    #[serde(with = "serde_str")]
    KeyId(multicipher::MKeyId),
    #[serde(with = "serde_str")]
    PublicKey(multicipher::MPublicKey),
}

impl PartialEq for Authentication {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Authentication::KeyId(id) => match other {
                Authentication::KeyId(other_id) => *id == *other_id,
                Authentication::PublicKey(other_key) => other_key.validate_id(id),
            },
            Authentication::PublicKey(key) => match other {
                Authentication::KeyId(other_id) => key.validate_id(other_id),
                Authentication::PublicKey(other_key) => *key == *other_key,
            },
        }
    }
}

impl FromStr for Authentication {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let auth = serde_json::from_value(serde_json::Value::String(s.to_owned()))?;
        Ok(auth)
    }
}

impl fmt::Display for Authentication {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::KeyId(id) => id.fmt(f),
            Self::PublicKey(key) => key.fmt(f),
        }
    }
}
