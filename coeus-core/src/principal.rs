use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Principal {
    #[serde(with = "serde_strz")]
    System(SystemPrincipal),
    #[serde(with = "serde_strz")]
    PublicKey(MPublicKey),
    #[cfg(feature = "did")]
    #[serde(with = "serde_strz")]
    Did(Did),
}

impl Principal {
    pub fn system() -> Self {
        Self::System(SystemPrincipal())
    }

    pub fn public_key(input: &str) -> Result<Self> {
        Ok(Principal::PublicKey(MPublicKey::from_str(input)?))
    }

    #[cfg(feature = "did")]
    pub fn did(input: &str) -> Result<Self> {
        Ok(Principal::Did(Did::from_str(input)?))
    }

    pub fn validate(&self, pk: &MPublicKey) -> Result<()> {
        match self {
            Self::System(_) => bail!("System principal cannot be impersonated"),
            Self::PublicKey(mypk) => {
                ensure!(
                    mypk == pk,
                    "PublicKey principal {} cannot be impersonated by {}",
                    mypk,
                    pk
                );
                Ok(())
            }
            #[cfg(feature = "did")]
            // Self::Did(mydid) => get_did_document(mydid).hasRight(pk, Impersonation)
            Self::Did(_) => todo!(),
        }
    }
}

/// Equal Principals will result in equal hash, so we are fine here
#[allow(clippy::derive_hash_xor_eq)]
impl Hash for Principal {
    fn hash<H>(&self, h: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.to_string().hash(h)
    }
}

impl fmt::Display for Principal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::System(s) => s.fmt(f),
            Self::PublicKey(pk) => pk.fmt(f),
            #[cfg(feature = "did")]
            Self::Did(did) => did.fmt(f),
        }
    }
}

impl FromStr for Principal {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Self> {
        let principal = serde_json::from_value(serde_json::Value::String(input.to_owned()))?;
        Ok(principal)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SystemPrincipal();

impl SystemPrincipal {
    const REPR: &'static str = "system";
}

impl fmt::Display for SystemPrincipal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(Self::REPR)
    }
}

impl FromStr for SystemPrincipal {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Self> {
        if input == Self::REPR {
            Ok(Self())
        } else {
            bail!("Expected '{}', but received {}", Self::REPR, input);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn serde_roundtrip(principal: Principal, expected: &str) {
        let serialized = serde_json::to_value(&principal).unwrap();

        assert_eq!(serialized, serde_json::Value::String(expected.to_owned()));

        let deserialized: Principal = serde_json::from_value(serialized).unwrap();

        assert_eq!(deserialized, principal);
    }

    #[test]
    fn serde_sys() {
        serde_roundtrip(Principal::system(), SystemPrincipal::REPR);
    }

    #[test]
    fn serde_pk() {
        let pk = "pez2CLkBUjHB8w8G87D3YkREjpRuiqPu6BrRsgHMQy2Pzt6";
        serde_roundtrip(Principal::public_key(pk).unwrap(), pk);
    }

    #[cfg(feature = "did")]
    #[test]
    fn serde_did() {
        let did = "did:morpheus:ezqztJ6XX6GDxdSgdiySiT3J";
        serde_roundtrip(Principal::did(did).unwrap(), did);
    }
}
