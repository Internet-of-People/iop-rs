use super::*;

/// Multicipher [`PublicKey`]
///
/// [`PublicKey`]: ../trait.AsymmetricCrypto.html#associatedtype.PublicKey
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MPublicKey {
    /// The public key tagged with this variant belongs to the [`ed25519`] module
    ///
    /// [`ed25519`]: ../ed25519/index.html
    Ed25519(EdPublicKey),
    /// The public key tagged with this variant belongs to the [`secp256k1`] module
    ///
    /// [`secp256k1`]: ../secp256k1/index.html
    Secp256k1(SecpPublicKey),
}

impl MPublicKey {
    /// All multicipher public keys start with this prefix
    pub const PREFIX: char = 'p';

    /// The ciphersuite that this public key belongs to
    pub fn suite(&self) -> CipherSuite {
        match self {
            Self::Ed25519(_) => CipherSuite::Ed25519,
            Self::Secp256k1(_) => CipherSuite::Secp256k1,
        }
    }

    /// Even the binary representation of a multicipher public key is readable with this.
    // TODO Should we really keep it like this?
    pub fn to_bytes(&self) -> Vec<u8> {
        String::from(self).as_bytes().to_vec()
    }

    /// Even the binary representation of a multicipher public key is readable with this.
    // TODO Should we really keep it like this?
    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> Result<Self> {
        let string = String::from_utf8(bytes.as_ref().to_owned())?;
        string.parse()
    }

    fn to_inner_bytes(&self) -> Vec<u8> {
        match self {
            Self::Ed25519(edpk) => edpk.to_bytes(),
            Self::Secp256k1(secppk) => secppk.to_bytes(),
        }
    }

    fn from_inner_bytes<B: AsRef<[u8]>>(suite: char, inner_bytes: B) -> Result<Self> {
        match CipherSuite::from_char(suite)? {
            CipherSuite::Ed25519 => Ok(Self::Ed25519(EdPublicKey::from_bytes(inner_bytes)?)),
            CipherSuite::Secp256k1 => Ok(Self::Secp256k1(SecpPublicKey::from_bytes(inner_bytes)?)),
        }
    }
}

impl PublicKey<MultiCipher> for MPublicKey {
    fn key_id(&self) -> MKeyId {
        match self {
            Self::Ed25519(edpk) => MKeyId::from(edpk.key_id()),
            Self::Secp256k1(secppk) => MKeyId::from(secppk.key_id()),
        }
    }

    fn validate_id(&self, key_id: &MKeyId) -> bool {
        match self {
            Self::Ed25519(edpk) => {
                if let MKeyId::Ed25519(edid) = key_id {
                    return edpk.validate_id(edid);
                }
            }
            Self::Secp256k1(secppk) => {
                if let MKeyId::Secp256k1(secpid) = key_id {
                    return secppk.validate_id(secpid);
                }
            }
        };

        false
    }

    fn verify<D: AsRef<[u8]>>(&self, data: D, sig: &MSignature) -> bool {
        match self {
            Self::Ed25519(edpk) => {
                if let MSignature::Ed25519(edsig) = sig {
                    return edpk.verify(data, edsig);
                }
            }
            Self::Secp256k1(secppk) => {
                if let MSignature::Secp256k1(secpsig) = sig {
                    return secppk.verify(data, secpsig);
                }
            }
        };

        false
    }
}

impl Serialize for MPublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let erased = ErasedBytes { suite: self.suite().as_byte(), value: self.to_inner_bytes() };
        erased.serialize(serializer)
    }
}

fn deser(erased: ErasedBytes) -> Result<MPublicKey> {
    MPublicKey::from_inner_bytes(erased.suite as char, &erased.value)
}

impl<'de> Deserialize<'de> for MPublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        ErasedBytes::deserialize(deserializer)
            .and_then(|b| deser(b).map_err(|e| serde::de::Error::custom(e.to_string())))
    }
}

impl From<&MPublicKey> for String {
    fn from(src: &MPublicKey) -> Self {
        let mut output = multibase::encode(multibase::Base::Base58Btc, src.to_inner_bytes());
        output.insert(0, src.suite().as_char());
        output.insert(0, MPublicKey::PREFIX);
        output
    }
}

impl From<MPublicKey> for String {
    fn from(src: MPublicKey) -> Self {
        (&src).into()
    }
}

impl std::fmt::Display for MPublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl std::fmt::Debug for MPublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        (self as &dyn std::fmt::Display).fmt(f)
    }
}

impl std::str::FromStr for MPublicKey {
    type Err = anyhow::Error;
    fn from_str(src: &str) -> Result<Self> {
        let mut chars = src.chars();
        ensure!(
            chars.next() == Some(Self::PREFIX),
            "Public keys must start with '{}'",
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

impl From<EdPublicKey> for MPublicKey {
    fn from(src: EdPublicKey) -> Self {
        Self::Ed25519(src)
    }
}

impl From<SecpPublicKey> for MPublicKey {
    fn from(src: SecpPublicKey) -> Self {
        Self::Secp256k1(src)
    }
}

#[cfg(test)]
mod test {
    mod parse_public_key {
        use crate::ed25519::EdPublicKey;
        use crate::multicipher::{CipherSuite, MPublicKey};

        #[allow(dead_code)]
        fn case(input: &str, pk_hex: &str) {
            let pk_bytes = hex::decode(pk_hex).unwrap();
            let pk1 = EdPublicKey::from_bytes(&pk_bytes).unwrap();
            let erased_pk1 = MPublicKey::from(pk1);
            assert_eq!(erased_pk1.to_string(), input);

            let erased_pk2 = input.parse::<MPublicKey>().unwrap();
            assert_eq!(erased_pk2, erased_pk1);
        }

        #[test]
        fn test_1() {
            case(
                "pez11111111111111111111111111111111",
                "0000000000000000000000000000000000000000000000000000000000000000",
            );
        }

        #[test]
        fn test_2() {
            case(
                "pezAgmjPHe5Qs4VakvXHGnd6NsYjaxt4suMUtf39TayrSfb",
                "8fe9693f8fa62a4305a140b9764c5ee01e455963744fe18204b4fb948249308a",
            );
        }

        #[test]
        fn test_3() {
            case(
                "pezFVen3X669xLzsi6N2V91DoiyzHzg1uAgqiT8jZ9nS96Z",
                "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
            );
        }

        #[test]
        fn test_4() {
            case(
                "pez586Z7H2vpX9qNhN2T4e9Utugie3ogjbxzGaMtM3E6HR5",
                "3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c",
            );
        }

        #[test]
        fn test_5() {
            case(
                "pezHyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr",
                "fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025",
            );
        }

        #[test]
        fn ed_suite() {
            let pk = "pez11111111111111111111111111111111".parse::<MPublicKey>().unwrap();
            assert_eq!(pk.suite(), CipherSuite::Ed25519);
            assert!(matches!(&pk, MPublicKey::Ed25519(_)));
            assert!(!matches!(&pk, MPublicKey::Secp256k1(_)));
        }

        #[test]
        fn secp_suite() {
            let pk =
                "psz291QGsvwafGPkKMu6MUsXThWRcBRzRf6pcVPM1Pst6WgW".parse::<MPublicKey>().unwrap();
            assert_eq!(pk.suite(), CipherSuite::Secp256k1);
            assert!(!matches!(&pk, MPublicKey::Ed25519(_)));
            assert!(matches!(&pk, MPublicKey::Secp256k1(_)));
        }

        #[test]
        #[should_panic(expected = "Unknown crypto suite 'g'")]
        fn invalid_suite() {
            let _pk =
                "pgzAgmjPHe5Qs4VakvXHGnd6NsYjaxt4suMUtf39TayrSfb".parse::<MPublicKey>().unwrap();
        }

        #[test]
        #[should_panic(expected = "No crypto suite found")]
        fn missing_suite() {
            let _pk = "p".parse::<MPublicKey>().unwrap();
        }

        #[test]
        #[should_panic(expected = "Public keys must start with 'p'")]
        fn invalid_type() {
            let _pk = "Fez21JXEtMzXjbCK6BAYFU9ewX".parse::<MPublicKey>().unwrap();
        }

        #[test]
        #[should_panic(expected = "Public keys must start with 'p'")]
        fn empty() {
            let _pk = "".parse::<MPublicKey>().unwrap();
        }
    }

    mod serde_key_id {
        use crate::multicipher::MPublicKey;

        #[test]
        fn messagepack_serialization() {
            let pk_str = "pezAgmjPHe5Qs4VakvXHGnd6NsYjaxt4suMUtf39TayrSfb";
            let pk = pk_str.parse::<MPublicKey>().unwrap();
            let pk_bin = rmp_serde::to_vec(&pk).unwrap();

            assert_eq!(
                pk_bin,
                vec![
                    146, 101, 196, 32, 143, 233, 105, 63, 143, 166, 42, 67, 5, 161, 64, 185, 118,
                    76, 94, 224, 30, 69, 89, 99, 116, 79, 225, 130, 4, 180, 251, 148, 130, 73, 48,
                    138
                ]
            );

            let pk_deser: MPublicKey = rmp_serde::from_slice(&pk_bin).unwrap();
            let pk_tostr = pk_deser.to_string();
            assert_eq!(pk, pk_deser);
            assert_eq!(pk_str, pk_tostr);
        }

        #[test]
        fn json_serialization() {
            let pk_str = "pezAgmjPHe5Qs4VakvXHGnd6NsYjaxt4suMUtf39TayrSfb";
            let pk = pk_str.parse::<MPublicKey>().unwrap();
            let pk_bin = serde_json::to_vec(&pk).unwrap();

            assert_eq!(pk_bin, br#"{"s":101,"v":[143,233,105,63,143,166,42,67,5,161,64,185,118,76,94,224,30,69,89,99,116,79,225,130,4,180,251,148,130,73,48,138]}"#.to_vec());

            let pk_deser: MPublicKey = serde_json::from_slice(&pk_bin).unwrap();
            let pk_tostr = pk_deser.to_string();
            assert_eq!(pk, pk_deser);
            assert_eq!(pk_str, pk_tostr);
        }
    }
}
