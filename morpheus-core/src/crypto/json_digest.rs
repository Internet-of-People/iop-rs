use failure::{bail, Fallible};

use crate::util::json_path;
use std::collections::HashMap;

pub fn hash_str(content: &str) -> String {
    // TODO we might want to use sha3 crate instead of tiny_keccak
    let mut hasher = tiny_keccak::Keccak::new_sha3_256();
    let mut hash_output = [0u8; 32];
    hasher.update(content.as_bytes());
    hasher.finalize(&mut hash_output);
    let mask = multibase::encode(multibase::Base::Base64Url, &hash_output);
    format!("cj{}", mask)
}

pub fn canonical_json(data: &serde_json::Value) -> String {
    match data {
        serde_json::Value::Array(arr) => {
            let mut canonical_json_items = Vec::new();
            for item in arr {
                canonical_json_items.push(canonical_json(item));
            }
            format!("[{}]", canonical_json_items.join(","))
        }

        serde_json::Value::Object(obj) => {
            let mut canonical_json_entries = Vec::new();
            let mut keys: Vec<_> = obj.keys().collect();
            keys.sort();
            for key in keys {
                let value = obj.get(key).expect("serde_json keys() impl error");
                let canonical_key = canonical_json(&serde_json::Value::String(key.to_owned()));
                let entry = format!("{}:{}", canonical_key, canonical_json(value));
                canonical_json_entries.push(entry);
            }
            // NOTE: braces are escaped as double brace in Rust
            format!("{{{}}}", canonical_json_entries.join(","))
        }

        _ => serde_json::to_string(data).expect("serde_json implementation error"),
    }
}

pub fn collapse_json_subtree(
    data: &serde_json::Value, keep_paths: Vec<&str>,
) -> Fallible<serde_json::Value> {
    match data {
        // NOTE path expressions are not (yet?) supported for arrays
        serde_json::Value::Array(arr) => {
            let mut canonical_json_items = Vec::new();
            for item in arr {
                let masked_item = collapse_json_subtree(item, vec![])?;
                canonical_json_items.push(serde_json::to_string(&masked_item)?);
            }
            let flattened_array = format!("[{}]", canonical_json_items.join(","));
            //println!("Flattened array {} to {}", serde_json::to_string(&data)?, flattened_array);
            let content_hash = hash_str(&flattened_array);
            Ok(serde_json::Value::String(content_hash))
        }

        serde_json::Value::Object(obj) => {
            // Build { head => vec![tails] } map
            let mut keep_head_tails = HashMap::new();
            for path in keep_paths {
                let (head, tail_opt) = json_path::split_head_tail(path)?;
                let tails = keep_head_tails.entry(head.to_owned()).or_insert(Vec::new());
                if let Some(tail) = tail_opt {
                    tails.push(tail);
                }
            }

            let mut collapse_root = true;
            let mut canonical_json_entries = Vec::new();
            let mut keys: Vec<_> = obj.keys().collect();
            keys.sort();
            for key in keys {
                let value = obj.get(key).expect("serde_json keys() impl error");
                if let Some(tails) = keep_head_tails.get(key) {
                    // Found object key present in keep_paths option, skip collapsing current branch of tree
                    collapse_root = false;
                    if tails.is_empty() {
                        // This is the exact Json path to keep open, do not collapse anything
                        canonical_json_entries.push((key, value.to_owned()));
                    } else {
                        // This is a partial match for a Json path to keep open, recurse to collapse it partially
                        let partial_value = collapse_json_subtree(value, tails.to_owned())?;
                        canonical_json_entries.push((key, partial_value));
                    }
                } else {
                    // This path does not match any paths, fully collapse
                    let fully_masked_value = collapse_json_subtree(value, vec![])?;
                    canonical_json_entries.push((key, fully_masked_value));
                };
            }

            if collapse_root {
                let canonical_entry_strs = canonical_json_entries
                    .iter()
                    // unwrap() also could be .expect("serde_json can't transform its own type into string")
                    .map(|(key, val)| {
                        let canonical_key = canonical_json(&serde_json::Value::String(key.to_string()));
                        format!("{}:{}", canonical_key, serde_json::to_string(val).unwrap())
                    })
                    .collect::<Vec<_>>();
                // NOTE: braces are escaped as double brace in Rust
                let flattened_object = format!("{{{}}}", canonical_entry_strs.join(","));

                let content_hash = hash_str(&flattened_object);
                Ok(serde_json::Value::String(content_hash))
            } else {
                let mut properties = serde_json::Map::new();
                for (key, value) in canonical_json_entries {
                    properties.insert(key.to_owned(), value);
                }
                Ok(serde_json::Value::Object(properties))
            }
        }

        _ => Ok(data.clone()),
    }
}

pub fn mask_json<T: serde::Serialize>(data: &T, keep_paths_str: &str) -> Fallible<String> {
    let keep_paths_vec = json_path::split_alternatives(keep_paths_str);
    let json_value = serde_json::to_value(&data)?;
    let digest_json = match &json_value {
        serde_json::Value::Object(_obj) => collapse_json_subtree(&json_value, keep_paths_vec),
        serde_json::Value::Array(_arr) => collapse_json_subtree(&json_value, keep_paths_vec),
        _ => bail!("Json digest is currently implemented only for composite types"),
    }?;
    match digest_json {
        serde_json::Value::String(digest) => return Ok(digest),
        serde_json::Value::Object(_) => return Ok(canonical_json(&digest_json)),
        _ => bail!("Implementation error: digest should always return a string or object"),
    }
}

pub fn json_digest<T: serde::Serialize>(data: &T) -> Fallible<String> {
    mask_json(data, "")
}

#[cfg(test)]
mod tests {
    use super::*;

    use failure::Fallible;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    struct TestData {
        b: u32,
        a: u32,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    struct CompositeTestData<T> {
        z: Option<T>,
        y: Option<T>,
    }

    #[test]
    fn test_json_digest() -> Fallible<()> {
        let test_obj = TestData { b: 1, a: 2 };
        {
            let masked = json_digest(&test_obj)?;
            assert_eq!(masked, "cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU");
        }
        {
            let masked = json_digest(&[&test_obj, &test_obj])?;
            assert_eq!(masked, "cjuGkDpb1HL7F8xFKDFVj3felfKZzjrJy92-108uuPixNw");
        }
        {
            let masked =
                json_digest(&(&test_obj, "cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU"))?;
            assert_eq!(masked, "cjuGkDpb1HL7F8xFKDFVj3felfKZzjrJy92-108uuPixNw");
        }
        {
            let masked = json_digest(&[
                "cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU",
                "cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU",
            ])?;
            assert_eq!(masked, "cjuGkDpb1HL7F8xFKDFVj3felfKZzjrJy92-108uuPixNw");
        }
        {
            let comp = CompositeTestData { z: Some(test_obj.clone()), y: Some(test_obj.clone()) };
            let masked = json_digest(&comp)?;
            assert_eq!(masked, "cjubdcpA0FfHhD8yEpDzZ8vS5sm7yxkrX_wAJgmke2bWRQ");
        }
        {
            let comp = CompositeTestData {
                z: Some("cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU".to_owned()),
                y: Some("cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU".to_owned()),
            };
            let masked = json_digest(&comp)?;
            assert_eq!(masked, "cjubdcpA0FfHhD8yEpDzZ8vS5sm7yxkrX_wAJgmke2bWRQ");
        }
        Ok(())
    }

    #[test]
    fn test_selective_masking() -> Fallible<()> {
        let test_obj = TestData { b: 1, a: 2 };
        let composite = CompositeTestData { z: Some(test_obj.clone()), y: Some(test_obj.clone()) };
        let double_complex =
            CompositeTestData { z: Some(composite.clone()), y: Some(composite.clone()) };
        let triple_complex =
            CompositeTestData { z: Some(double_complex.clone()), y: Some(double_complex.clone()) };
        {
            let fully_masked = mask_json(&composite, "")?;
            assert_eq!(fully_masked, "cjubdcpA0FfHhD8yEpDzZ8vS5sm7yxkrX_wAJgmke2bWRQ");
        }
        {
            let keep_y = mask_json(&composite, ".y")?;
            assert_eq!(
                keep_y,
                r#"{"y":{"a":2,"b":1},"z":"cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU"}"#
            );
            let val: serde_json::Value = serde_json::from_str(&keep_y)?;
            assert_eq!(json_digest(&val)?, "cjubdcpA0FfHhD8yEpDzZ8vS5sm7yxkrX_wAJgmke2bWRQ");
        }
        {
            let keep_z = mask_json(&composite, ".z")?;
            assert_eq!(
                keep_z,
                r#"{"y":"cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU","z":{"a":2,"b":1}}"#
            );
            let val: serde_json::Value = serde_json::from_str(&keep_z)?;
            assert_eq!(json_digest(&val)?, "cjubdcpA0FfHhD8yEpDzZ8vS5sm7yxkrX_wAJgmke2bWRQ");
        }
        {
            let digest = json_digest(&double_complex)?;
            assert_eq!(digest, "cjuQLebyl_BJipFLibhWiStDBqK5J4JZq15ehUqybfTTKA");
        }
        {
            let keep_yz = mask_json(&double_complex, ".y.z")?;
            assert_eq!(
                keep_yz,
                r#"{"y":{"y":"cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU","z":{"a":2,"b":1}},"z":"cjubdcpA0FfHhD8yEpDzZ8vS5sm7yxkrX_wAJgmke2bWRQ"}"#
            );
            let val: serde_json::Value = serde_json::from_str(&keep_yz)?;
            assert_eq!(json_digest(&val)?, "cjuQLebyl_BJipFLibhWiStDBqK5J4JZq15ehUqybfTTKA");
        }
        {
            let digest = json_digest(&triple_complex)?;
            assert_eq!(digest, "cjuik140L3w7LCi6z1eHt7Qgwr2X65-iy8HA6zqrlUdmVk");
        }
        {
            let keep_yz = mask_json(&triple_complex, ".y.y , .z.z")?;
            assert_eq!(
                keep_yz,
                r#"{"y":{"y":{"y":{"a":2,"b":1},"z":{"a":2,"b":1}},"z":"cjubdcpA0FfHhD8yEpDzZ8vS5sm7yxkrX_wAJgmke2bWRQ"},"z":{"y":"cjubdcpA0FfHhD8yEpDzZ8vS5sm7yxkrX_wAJgmke2bWRQ","z":{"y":{"a":2,"b":1},"z":{"a":2,"b":1}}}}"#
            );
            let val: serde_json::Value = serde_json::from_str(&keep_yz)?;
            assert_eq!(json_digest(&val)?, "cjuik140L3w7LCi6z1eHt7Qgwr2X65-iy8HA6zqrlUdmVk");
        }
        Ok(())
    }
}
