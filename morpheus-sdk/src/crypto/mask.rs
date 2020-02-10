pub fn canonical_json(data: &serde_json::Value) -> serde_json::Result<String> {
    match data {
        serde_json::Value::Array(arr) => {
            let mut canonical_json_items = Vec::new();
            for item in arr {
                canonical_json_items.push(canonical_json(item)?);
            }
            let can_items_str = canonical_json_items.join(",");
            Ok(format!("[{}]", can_items_str))
        }

        serde_json::Value::Object(obj) => {
            let mut canonical_json_entries = Vec::new();
            let mut keys: Vec<_> = obj.keys().collect();
            keys.sort();
            for key in keys {
                let value = obj.get(key).expect("serde_json keys() impl error");
                let canonical_value = canonical_json(value)?;
                let entry = format!(
                    "{}:{}",
                    canonical_json(&serde_json::Value::String(key.to_owned()))?,
                    canonical_value
                );
                canonical_json_entries.push(entry);
            }
            let can_entries_str = canonical_json_entries.join(",");
            Ok(format!("{{{}}}", can_entries_str)) // NOTE: escape enclosing braces as doubles in Rust
        }

        _ => serde_json::to_string(data),
    }
}

pub fn digest(content: &str) -> String {
    let mut hasher = tiny_keccak::Keccak::new_sha3_256();
    let mut hash_output = [0u8; 32];
    hasher.update(content.as_bytes());
    hasher.finalize(&mut hash_output);
    let mask = multibase::encode(multibase::Base::Base64UrlUpperNoPad, &hash_output);
    format!("cj{}", mask)
}

pub fn merkle_json(data: &serde_json::Value) -> serde_json::Result<String> {
    match data {
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
            let canonical_json = canonical_json(data)?;
            Ok(digest(&canonical_json))
        }
        _ => canonical_json(data),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use failure::Fallible;
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    struct TestData {
        b: u32,
        a: u32,
    }

    #[test]
    fn masking() -> Fallible<()> {
        let test_obj = TestData { b: 1, a: 2 };
        let value = serde_json::to_value(test_obj)?;
        let mask = merkle_json(&value)?;
        assert_eq!(mask, "cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU");
        //println!("Canonical JSON: {}", canonical_json(&value)?);
        //println!("Masked {} object into {}", value, mask);
        Ok(())
    }
}

//const o = {b: 1, a: 2}
//> jsonDigest([o, o])
//{"a":2,"b":1}
//{"a":2,"b":1}
//'["cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU","cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU"]'
//> jsonDigest({z: o, y: o})
//{"a":2,"b":1}
//{"a":2,"b":1}
//{"y":"cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU","z":"cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU"}
//'"cjubdcpA0FfHhD8yEpDzZ8vS5sm7yxkrX_wAJgmke2bWRQ"'
//> jsonDigest({z: o, y: 'cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU'})
//{"a":2,"b":1}
//{"y":"cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU","z":"cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU"}
//'"cjubdcpA0FfHhD8yEpDzZ8vS5sm7yxkrX_wAJgmke2bWRQ"'
