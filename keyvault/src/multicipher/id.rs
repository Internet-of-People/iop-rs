use super::*;

/// Multicipher [`KeyId`]
///
/// [`KeyId`]: ../trait.AsymmetricCrypto.html#associatedtype.KeyId
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MKeyId {
    /// The key id tagged with this variant belongs to the [`ed25519`] module
    ///
    /// [`ed25519`]: ../ed25519/index.html
    Ed25519(EdKeyId),
    /// The key id tagged with this variant belongs to the [`secp256k1`] module
    ///
    /// [`secp256k1`]: ../secp256k1/index.html
    Secp256k1(SecpKeyId),
}

impl MKeyId {
    /// All multicipher keyids start with this prefix
    pub const PREFIX: char = 'i';

    /// The ciphersuite that this key id belongs to
    pub fn suite(&self) -> CipherSuite {
        match self {
            Self::Ed25519(_) => CipherSuite::Ed25519,
            Self::Secp256k1(_) => CipherSuite::Secp256k1,
        }
    }

    /// Even the binary representation of a multicipher keyid is readable with this.
    // TODO Should we really keep it like this?
    pub fn to_bytes(&self) -> Vec<u8> {
        String::from(self).as_bytes().to_vec()
    }

    /// Even the binary representation of a multicipher keyid is readable with this.
    // TODO Should we really keep it like this?
    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> Result<Self> {
        let string = String::from_utf8(bytes.as_ref().to_owned())?;
        string.parse()
    }

    fn to_inner_bytes(&self) -> Vec<u8> {
        match self {
            Self::Ed25519(edid) => edid.to_bytes(),
            Self::Secp256k1(secpid) => secpid.to_bytes(),
        }
    }

    fn from_inner_bytes<B: AsRef<[u8]>>(suite: char, inner_bytes: B) -> Result<Self> {
        match CipherSuite::from_char(suite)? {
            CipherSuite::Ed25519 => Ok(Self::Ed25519(EdKeyId::from_bytes(inner_bytes)?)),
            CipherSuite::Secp256k1 => Ok(Self::Secp256k1(SecpKeyId::from_bytes(inner_bytes)?)),
        }
    }
}

impl Serialize for MKeyId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut bytes_out = self.to_inner_bytes();
        bytes_out.insert(0, self.suite().as_byte());
        serde_bytes::serialize(bytes_out.as_slice(), serializer)
    }
}

fn deser(bytes: Vec<u8>) -> Result<MKeyId> {
    ensure!(!bytes.is_empty(), "No crypto suite found");
    let suite = bytes[0] as char;
    let data = &bytes[1..];
    MKeyId::from_inner_bytes(suite, data)
}

impl<'de> Deserialize<'de> for MKeyId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_bytes::deserialize(deserializer)
            .and_then(|b| deser(b).map_err(|e| serde::de::Error::custom(e.to_string())))
    }
}

impl From<&MKeyId> for String {
    fn from(src: &MKeyId) -> Self {
        let mut output = multibase::encode(multibase::Base::Base58Btc, src.to_inner_bytes());
        output.insert(0, src.suite().as_char());
        output.insert(0, MKeyId::PREFIX);
        output
    }
}

impl From<MKeyId> for String {
    fn from(src: MKeyId) -> Self {
        (&src).into()
    }
}

impl std::fmt::Display for MKeyId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl std::fmt::Debug for MKeyId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        (self as &dyn std::fmt::Display).fmt(f)
    }
}

impl std::str::FromStr for MKeyId {
    type Err = anyhow::Error;
    fn from_str(src: &str) -> Result<Self> {
        let mut chars = src.chars();
        ensure!(
            chars.next() == Some(Self::PREFIX),
            "Identifiers must start with '{}'",
            Self::PREFIX
        );
        if let Some(suite) = chars.next() {
            let (_base, binary) = multibase::decode(chars.as_str())?;
            Self::from_inner_bytes(suite, &binary)
        } else {
            Err(anyhow!("No crypto suite found"))
        }
    }
}

impl From<EdKeyId> for MKeyId {
    fn from(src: EdKeyId) -> Self {
        Self::Ed25519(src)
    }
}

impl From<SecpKeyId> for MKeyId {
    fn from(src: SecpKeyId) -> Self {
        Self::Secp256k1(src)
    }
}

#[cfg(test)]
mod test {
    mod parse_key_id {
        use crate::ed25519::EdKeyId;
        use crate::multicipher::{CipherSuite, MKeyId};

        #[allow(dead_code)]
        fn case(input: &str, key_id_hex: &str) {
            let key_id_bytes = hex::decode(key_id_hex).unwrap();
            let id1 = EdKeyId::from_bytes(&key_id_bytes).unwrap();
            let erased_id1 = MKeyId::from(id1);
            assert_eq!(erased_id1.to_string(), input);

            let erased_id2 = input.parse::<MKeyId>().unwrap();
            assert_eq!(erased_id2, erased_id1);
        }

        #[test]
        fn test_1() {
            case("iez21JXEtMzXjbCK6BAYFU9ewX", "01d8245272e2317ef53b26407e925edf7e");
        }

        #[test]
        fn test_2() {
            case("iezpmXKKc2QRZpXbzGV62MgKe", "0182d4ecfc12c5ad8efa5ef494f47e5285");
        }

        #[test]
        fn ed_suite() {
            let id = "iez21JXEtMzXjbCK6BAYFU9ewX".parse::<MKeyId>().unwrap();
            assert_eq!(id.suite(), CipherSuite::Ed25519);
            assert!(matches!(&id, MKeyId::Ed25519(_)));
            assert!(!matches!(&id, MKeyId::Secp256k1(_)));
        }

        #[test]
        fn secp_suite() {
            let id = "isz7un9h2Ddua9rfHefJMKiPLxSy2pX".parse::<MKeyId>().unwrap();
            assert_eq!(id.suite(), CipherSuite::Secp256k1);
            assert!(!matches!(&id, MKeyId::Ed25519(_)));
            assert!(matches!(&id, MKeyId::Secp256k1(_)));
        }

        #[test]
        #[should_panic(expected = "Unknown crypto suite 'g'")]
        fn invalid_suite() {
            let _id = "igz21JXEtMzXjbCK6BAYFU9ewX".parse::<MKeyId>().unwrap();
        }

        #[test]
        #[should_panic(expected = "No crypto suite found")]
        fn missing_suite() {
            let _id = "i".parse::<MKeyId>().unwrap();
        }

        #[test]
        #[should_panic(expected = "Identifiers must start with 'i'")]
        fn invalid_type() {
            let _id = "fez21JXEtMzXjbCK6BAYFU9ewX".parse::<MKeyId>().unwrap();
        }

        #[test]
        #[should_panic(expected = "Identifiers must start with 'i'")]
        fn empty() {
            let _id = "".parse::<MKeyId>().unwrap();
        }
    }

    mod serde_key_id {
        use crate::multicipher::MKeyId;

        #[test]
        fn messagepack_serialization() {
            let id_str = "iez21JXEtMzXjbCK6BAYFU9ewX";
            let id = id_str.parse::<MKeyId>().unwrap();
            let id_bin = rmp_serde::to_vec(&id).unwrap();

            assert_eq!(
                id_bin,
                vec![
                    196, 18, 101, 1, 216, 36, 82, 114, 226, 49, 126, 245, 59, 38, 64, 126, 146, 94,
                    223, 126
                ]
            );

            let id_deser: MKeyId = rmp_serde::from_slice(&id_bin).unwrap();
            let id_tostr = id_deser.to_string();
            assert_eq!(id, id_deser);
            assert_eq!(id_str, id_tostr);
        }
    }
}
