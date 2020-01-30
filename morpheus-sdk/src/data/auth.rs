use std::convert::TryFrom;

use failure::{err_msg, Fallible};
use serde::{Deserialize, Serialize};

use keyvault::multicipher;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(try_from = "MAuthentication", into = "MAuthentication")]
pub enum Authentication {
    KeyId(multicipher::MKeyId),
    PublicKey(multicipher::MPublicKey),
}

#[derive(Deserialize, Serialize)]
#[serde(transparent)]
struct MAuthentication(String);

impl TryFrom<MAuthentication> for Authentication {
    type Error = failure::Error;

    fn try_from(value: MAuthentication) -> Fallible<Self> {
        if value.0.starts_with(multicipher::MKeyId::PREFIX) {
            let key_id = value.0.parse()?;
            Ok(Authentication::KeyId(key_id))
        } else if value.0.starts_with(multicipher::MPublicKey::PREFIX) {
            let pk = value.0.parse()?;
            Ok(Authentication::PublicKey(pk))
        } else {
            Err(err_msg(format!("Authentication starts with invalid character: {}", value.0)))
        }
    }
}

impl Into<MAuthentication> for Authentication {
    fn into(self) -> MAuthentication {
        let string = match self {
            Authentication::KeyId(key_id) => key_id.to_string(),
            Authentication::PublicKey(pk) => pk.to_string(),
        };
        MAuthentication(string)
    }
}

impl ToString for Authentication {
    fn to_string(&self) -> String {
        match self {
            Self::KeyId(id) => id.to_string(),
            Self::PublicKey(key) => key.to_string(),
        }
    }
}
