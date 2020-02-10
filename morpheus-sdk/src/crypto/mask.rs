pub fn canonical_json(data: &serde_json::Value) -> serde_json::Result<String> {
    match data {
        serde_json::Value::Array(arr) => {
            let mut canonical_json_items = Vec::new();
            for item in arr {
                canonical_json_items.push(canonical_json(item)?);
            }
            Ok(format!("[{}]", canonical_json_items.join(",")))
        }

        serde_json::Value::Object(obj) => {
            let mut canonical_json_entries = Vec::new();
            let mut keys: Vec<_> = obj.keys().collect();
            keys.sort();
            for key in keys {
                let value = obj.get(key).expect("serde_json keys() impl error");
                let canonical_key = canonical_json(&serde_json::Value::String(key.to_owned()))?;
                let entry = format!("{}:{}", canonical_key, canonical_json(value)?);
                canonical_json_entries.push(entry);
            }
            // NOTE: braces are escaped as double brace in Rust
            Ok(format!("{{{}}}", canonical_json_entries.join(",")))
        }

        _ => serde_json::to_string(data),
    }
}

pub fn hashed(content: &str) -> String {
    let mut hasher = tiny_keccak::Keccak::new_sha3_256();
    let mut hash_output = [0u8; 32];
    hasher.update(content.as_bytes());
    hasher.finalize(&mut hash_output);
    let mask = multibase::encode(multibase::Base::Base64UrlUpperNoPad, &hash_output);
    format!("cj{}", mask)
}

pub fn json_digest(data: &serde_json::Value) -> serde_json::Result<String> {
    match data {
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
            let canonical_json = canonical_json(data)?;
            let digest = hashed(&canonical_json);
            // println!("Digest of {} is {}", canonical_json, digest);
            Ok(digest)
        }
        _ => canonical_json(data),
    }
}

pub fn mask_json_value(data: &serde_json::Value) -> serde_json::Result<serde_json::Value> {
    match data {
        serde_json::Value::Array(arr) => {
            let mut canonical_json_items = Vec::new();
            for item in arr {
                let masked_item = mask_json_value(item)?;
                canonical_json_items.push(serde_json::to_string(&masked_item)?);
            }
            let flattened_array = format!("[{}]", canonical_json_items.join(","));
            //println!("Flattened array {} to {}", serde_json::to_string(&data)?, flattened_array);
            let content_hash = hashed(&flattened_array);
            Ok(serde_json::Value::String(content_hash))
        }

        serde_json::Value::Object(obj) => {
            let mut canonical_json_entries = Vec::new();
            let mut keys: Vec<_> = obj.keys().collect();
            keys.sort();
            for key in keys {
                let value = obj.get(key).expect("serde_json keys() impl error");
                let canonical_key = canonical_json(&serde_json::Value::String(key.to_owned()))?;
                let masked_val = mask_json_value(value)?;
                let entry = format!("{}:{}", canonical_key, serde_json::to_string(&masked_val)?);
                canonical_json_entries.push(entry);
            }
            // NOTE: braces are escaped as double brace in Rust
            let flattened_object = format!("{{{}}}", canonical_json_entries.join(","));
            //println!("Flattened object {} to {}", serde_json::to_string(&data)?, flattened_object);
            let content_hash = hashed(&flattened_object);
            Ok(serde_json::Value::String(content_hash))
        }

        _ => Ok(data.clone()),
    }
}

pub fn mask<T: serde::Serialize>(data: &T) -> serde_json::Result<String> {
    let json_value = serde_json::to_value(&data)?;
    serde_json::to_string(&mask_json_value(&json_value)?)
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
        {
            let masked = mask(&test_obj)?;
            assert_eq!(masked, r#""cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU""#);
        }
        {
            let masked = mask(&[&test_obj, &test_obj])?;
            assert_eq!(masked, r#""cjuGkDpb1HL7F8xFKDFVj3felfKZzjrJy92-108uuPixNw""#);
        }
        {
            let masked = mask(&(&test_obj, "cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU"))?;
            assert_eq!(masked, r#""cjuGkDpb1HL7F8xFKDFVj3felfKZzjrJy92-108uuPixNw""#);
        }
        {
            let masked = mask(&[
                "cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU",
                "cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU",
            ])?;
            assert_eq!(masked, r#""cjuGkDpb1HL7F8xFKDFVj3felfKZzjrJy92-108uuPixNw""#);
        }
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
