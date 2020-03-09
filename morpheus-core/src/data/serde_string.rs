use serde::{de::Visitor, Deserializer, Serializer};
use std::fmt;
use std::str::FromStr;

struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(value.to_owned())
    }
}

pub fn serialize<T: ToString, S: Serializer>(t: &T, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&t.to_string())
}

pub fn deserialize<'de, T: FromStr, D: Deserializer<'de>>(d: D) -> Result<T, D::Error> {
    let s = d.deserialize_any(StringVisitor)?;
    FromStr::from_str(&s).map_err(|_e: T::Err| {
        serde::de::Error::invalid_value(serde::de::Unexpected::Str(&s), &"Parsable to T")
    })
}
