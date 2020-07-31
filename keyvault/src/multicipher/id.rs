use failure::{ensure, err_msg, Fallible};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;

use super::*;

erased_type! {
    /// Type-erased [`KeyId`]
    ///
    /// [`KeyId`]: ../trait.AsymmetricCrypto.html#associatedtype.KeyId
    pub struct MKeyId {}
}

impl MKeyId {
    /// All multicipher keyids start with this prefix
    pub const PREFIX: char = 'i';
}

macro_rules! to_bytes_tuple {
    ($suite:ident, $self_:expr) => {
        (stringify!($suite), reify!($suite, id, $self_).to_bytes())
    };
}

impl Serialize for MKeyId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (suite, bytes) = visit!(to_bytes_tuple(self));
        let mut out = bytes;
        out.insert(0, suite.as_bytes()[0]);
        serde_bytes::serialize(out.as_slice(), serializer)
    }
}

macro_rules! from_bytes {
    ($suite:ident, $data:expr) => {
        erase!($suite, MKeyId, <$suite!(id)>::from_bytes($data)?)
    };
}

fn deser(bytes: Vec<u8>) -> Fallible<MKeyId> {
    ensure!(!bytes.is_empty(), "No crypto suite suite found");
    let suite = bytes[0] as char;
    let data = &bytes[1..];
    let value = visit_fac!(
        stringify(suite.to_string().as_str()) =>
            from_bytes(data)
    );
    Ok(value)
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

macro_rules! clone {
    ($suite:ident, $self_:expr) => {{
        let result = reify!($suite, id, $self_).clone();
        erase!($suite, MKeyId, result)
    }};
}

impl Clone for MKeyId {
    fn clone(&self) -> Self {
        visit!(clone(self))
    }
}

macro_rules! eq {
    ($suite:ident, $self_:tt, $other:ident) => {
        reify!($suite, id, $self_).eq(reify!($suite, id, $other))
    };
}

impl PartialEq<MKeyId> for MKeyId {
    fn eq(&self, other: &Self) -> bool {
        if self.suite != other.suite {
            return false;
        }
        visit!(eq(self, other))
    }
}

impl Eq for MKeyId {}

macro_rules! partial_cmp {
    ($suite:ident, $self_:tt, $other:expr) => {
        reify!($suite, id, $self_).partial_cmp(reify!($suite, id, $other))
    };
}

impl PartialOrd<MKeyId> for MKeyId {
    fn partial_cmp(&self, other: &MKeyId) -> Option<Ordering> {
        let suite_order = self.suite.partial_cmp(&other.suite);
        match suite_order {
            Some(Ordering::Equal) => visit!(partial_cmp(self, other)),
            _ => suite_order,
        }
    }
}

macro_rules! hash {
    ($suite:ident, $self_:tt, $state:expr) => {
        reify!($suite, id, $self_).hash($state)
    };
}

impl Hash for MKeyId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.suite.hash(state);
        visit!(hash(self, state));
    }
}

impl From<&MKeyId> for String {
    fn from(src: &MKeyId) -> Self {
        let (suite, bytes) = visit!(to_bytes_tuple(src));
        let mut output = multibase::encode(multibase::Base::Base58Btc, &bytes);
        output.insert_str(0, suite);
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
    type Err = failure::Error;
    fn from_str(src: &str) -> Fallible<Self> {
        let mut chars = src.chars();
        ensure!(
            chars.next() == Some(Self::PREFIX),
            "Identifiers must start with '{}'",
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
            Err(err_msg("No crypto suite suite found"))
        }
    }
}

impl From<EdKeyId> for MKeyId {
    fn from(src: EdKeyId) -> Self {
        erase!(e, MKeyId, src)
    }
}

impl From<SecpKeyId> for MKeyId {
    fn from(src: SecpKeyId) -> Self {
        erase!(s, MKeyId, src)
    }
}

impl MKeyId {
    /// Even the binary representation of a multicipher keyid is readable with this.
    // TODO Should we really keep it like this?
    pub fn to_bytes(&self) -> Vec<u8> {
        String::from(self).as_bytes().to_vec()
    }
    /// Even the binary representation of a multicipher keyid is readable with this.
    // TODO Should we really keep it like this?
    pub fn from_bytes(bytes: &[u8]) -> Fallible<Self> {
        let string = String::from_utf8(bytes.to_owned())?;
        string.parse()
    }
}

#[cfg(test)]
mod test {
    mod parse_key_id {
        use crate::ed25519::EdKeyId;
        use crate::multicipher::MKeyId;

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
        fn suite_matters() {
            let id1 = "iez21JXEtMzXjbCK6BAYFU9ewX".parse::<MKeyId>().unwrap();
            let id2 = "ifz21JXEtMzXjbCK6BAYFU9ewX".parse::<MKeyId>().unwrap();
            assert_ne!(id1, id2);
        }

        #[test]
        #[should_panic(expected = "Unknown crypto suite suite \\'g\\'")]
        fn invalid_suite() {
            let _id = "igz21JXEtMzXjbCK6BAYFU9ewX".parse::<MKeyId>().unwrap();
        }

        #[test]
        #[should_panic(expected = "No crypto suite suite found")]
        fn missing_suite() {
            let _id = "i".parse::<MKeyId>().unwrap();
        }

        #[test]
        #[should_panic(expected = "Identifiers must start with \\'i\\'")]
        fn invalid_type() {
            let _id = "fez21JXEtMzXjbCK6BAYFU9ewX".parse::<MKeyId>().unwrap();
        }

        #[test]
        #[should_panic(expected = "Identifiers must start with \\'i\\'")]
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
