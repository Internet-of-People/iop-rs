use super::*;

erased_type! {
    /// Type-erased [`PublicKey`]
    ///
    /// [`PublicKey`]: ../trait.AsymmetricCrypto.html#associatedtype.PublicKey
    pub struct MPublicKey {}
}

macro_rules! key_id {
    ($suite:ident, $self_:tt) => {{
        let result = reify!($suite, pk, $self_).key_id();
        erase!($suite, MKeyId, result)
    }};
}

macro_rules! verify {
    ($suite:ident, $self_:tt, $data:ident, $sig:ident) => {
        reify!($suite, pk, $self_).verify($data, reify!($suite, sig, $sig))
    };
}

impl MPublicKey {
    /// All multicipher public keys start with this prefix
    pub const PREFIX: char = 'p';

    /// Even the binary representation of a multicipher public key is readable with this.
    // TODO Should we really keep it like this?
    pub fn to_bytes(&self) -> Vec<u8> {
        String::from(self).as_bytes().to_vec()
    }

    /// Even the binary representation of a multicipher public key is readable with this.
    // TODO Should we really keep it like this?
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let string = String::from_utf8(bytes.to_owned())?;
        string.parse()
    }
}

impl PublicKey<MultiCipher> for MPublicKey {
    fn key_id(&self) -> MKeyId {
        visit!(key_id(self))
    }
    fn validate_id(&self, key_id: &MKeyId) -> bool {
        &self.key_id() == key_id
    }
    fn verify<D: AsRef<[u8]>>(&self, data: D, sig: &MSignature) -> bool {
        if self.suite != sig.suite {
            return false;
        }
        visit!(verify(self, data, sig))
    }
}

macro_rules! to_bytes_tuple {
    ($suite:ident, $self_:expr) => {
        (stringify!($suite), reify!($suite, pk, $self_).to_bytes())
    };
}

impl Serialize for MPublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (suite, bytes) = visit!(to_bytes_tuple(self));

        let erased = ErasedBytes { suite: suite.as_bytes()[0], value: bytes };
        erased.serialize(serializer)
    }
}

macro_rules! from_bytes {
    ($suite:ident, $data:expr) => {
        erase!($suite, MPublicKey, <$suite!(pk)>::from_bytes($data)?)
    };
}

fn deser(erased: ErasedBytes) -> Result<MPublicKey> {
    let suite = erased.suite as char;
    let data = &erased.value;
    let value = visit_fac!(
        stringify(suite.to_string().as_str()) =>
            from_bytes(data)
    );
    Ok(value)
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

macro_rules! clone {
    ($suite:ident, $self_:expr) => {{
        let result = reify!($suite, pk, $self_).clone();
        erase!($suite, MPublicKey, result)
    }};
}

impl Clone for MPublicKey {
    fn clone(&self) -> Self {
        visit!(clone(self))
    }
}

macro_rules! eq {
    ($suite:ident, $self_:tt, $other:ident) => {
        reify!($suite, pk, $self_).eq(reify!($suite, pk, $other))
    };
}

impl PartialEq<MPublicKey> for MPublicKey {
    fn eq(&self, other: &Self) -> bool {
        if self.suite != other.suite {
            return false;
        }
        visit!(eq(self, other))
    }
}

impl Eq for MPublicKey {}

macro_rules! cmp {
    ($suite:ident, $self_:tt, $other:expr) => {
        reify!($suite, pk, $self_).cmp(reify!($suite, pk, $other))
    };
}

impl PartialOrd<Self> for MPublicKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MPublicKey {
    fn cmp(&self, other: &Self) -> Ordering {
        let suite_order = self.suite.cmp(&other.suite);
        match suite_order {
            Ordering::Equal => visit!(cmp(self, other)),
            _ => suite_order,
        }
    }
}

macro_rules! hash {
    ($suite:ident, $self_:tt, $state:expr) => {
        reify!($suite, pk, $self_).hash($state)
    };
}

impl Hash for MPublicKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.suite.hash(state);
        visit!(hash(self, state));
    }
}

impl From<&MPublicKey> for String {
    fn from(src: &MPublicKey) -> Self {
        let (suite, bytes) = visit!(to_bytes_tuple(src));
        let mut output = multibase::encode(multibase::Base::Base58Btc, &bytes);
        output.insert_str(0, suite);
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
            let ret = visit_fac!(
                stringify(suite.to_string().as_str()) =>
                    from_bytes(binary)
            );
            Ok(ret)
        } else {
            Err(anyhow!("No crypto suite suite found"))
        }
    }
}

impl From<EdPublicKey> for MPublicKey {
    fn from(src: EdPublicKey) -> Self {
        erase!(e, MPublicKey, src)
    }
}

impl From<SecpPublicKey> for MPublicKey {
    fn from(src: SecpPublicKey) -> Self {
        erase!(s, MPublicKey, src)
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
        }

        #[test]
        fn secp_suite() {
            let pk =
                "psz291QGsvwafGPkKMu6MUsXThWRcBRzRf6pcVPM1Pst6WgW".parse::<MPublicKey>().unwrap();
            assert_eq!(pk.suite(), CipherSuite::Secp256k1);
        }

        #[test]
        #[should_panic(expected = "Unknown crypto suite suite 'g'")]
        fn invalid_suite() {
            let _pk =
                "pgzAgmjPHe5Qs4VakvXHGnd6NsYjaxt4suMUtf39TayrSfb".parse::<MPublicKey>().unwrap();
        }

        #[test]
        #[should_panic(expected = "No crypto suite suite found")]
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
