use super::*;

use tiny_keccak::Hasher;
use unicode_normalization::UnicodeNormalization;

/// Returns an [NFKD normalized] unicode representation of the input
///
/// [NFKD normalized]: https://en.wikipedia.org/wiki/Unicode_equivalence#Normal_forms
pub fn normalize_unicode(s: &str) -> String {
    s.nfkd().collect()
}

/// Multibase-encoded hash of the provided bytes used in many places around this crate.
///
/// We use SHA3_256 (not Keccak) with Base-64 URL encoding.
pub fn default_hasher(content: &[u8]) -> String {
    // TODO we might want to use sha3 crate instead of tiny_keccak
    let mut hasher = tiny_keccak::Sha3::v256();
    let mut hash_output = [0u8; 32];
    hasher.update(content);
    hasher.finalize(&mut hash_output);
    multibase::encode(multibase::Base::Base64Url, &hash_output)
}

/// Multibase-encoded hash of the utf8 representation of the provided string,
/// prefixed with "cj". Character 'j' marks that a JSON value was hashed and
/// 'c' stands for content hash.
fn hash_str(content: &str) -> String {
    format!("cj{}", default_hasher(content.as_bytes()))
}

/// Constructs the deterministic string representation of the provided JSON value.
///
/// The same JSON document can be represented in multiple ways depending on
/// property ordering and string encoding. This canonical JSON format lists
/// property names in ascending order by their utf8-encoded byte arrays and
/// uses the [NFKD normalized] unicode representation of all strings
/// (both property names and string values).
///
/// The function will return error if a provided JSON object has properties
/// with the same name in different unicode normalizations.
/// Note that creating `Value` arguments, `serde_json` accepts objects
/// having properties with exactly the same name and keeps only the last value,
/// ignoring all previous ones.
///
/// [NFKD normalized]: https://en.wikipedia.org/wiki/Unicode_equivalence#Normal_forms
pub fn canonical_json(data: &serde_json::Value) -> Result<String> {
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
                ensure!(
                    *key == normalize_unicode(key),
                    "Data for canonical JSON serialization must contain field names normalized with Unicode NFKD"
                );

                let value = obj.get(key).expect("serde_json keys() impl error");
                let canonical_key = canonical_json(&serde_json::Value::String(key.to_owned()))?;
                let entry = format!("{}:{}", canonical_key, canonical_json(value)?);
                canonical_json_entries.push(entry);
            }
            // NOTE: braces are escaped as double braces in Rust
            Ok(format!("{{{}}}", canonical_json_entries.join(",")))
        }

        _ => {
            let data_str = serde_json::to_string(data).expect("serde_json implementation error");
            Ok(normalize_unicode(&data_str))
        }
    }
}

/// Replace JSON (sub)tree(s) with their multibase-encoded [Merkle-root hash] strings.
///
/// Argument `keep_paths` can be created using function [`split_alternatives`] as needed.
///
/// [`split_alternatives`]: ../json_path/fn.split_alternatives.html
/// [Merkle-root hash]: https://en.wikipedia.org/wiki/Merkle_tree
pub fn mask_json_subtree<'a, 'b>(
    data: &'a serde_json::Value, keep_paths: impl AsRef<[&'b str]>,
) -> Result<serde_json::Value> {
    match data {
        // NOTE path expressions are not (yet?) supported for arrays
        serde_json::Value::Array(arr) => {
            let mut canonical_json_items = Vec::new();
            for item in arr {
                let digested_item = mask_json_subtree(item, vec![])?;
                canonical_json_items.push(serde_json::to_string(&digested_item)?);
            }
            let flattened_array = format!("[{}]", canonical_json_items.join(","));
            //println!("Flattened array {} to {}", serde_json::to_string(&data)?, flattened_array);
            let content_hash = hash_str(&flattened_array);
            Ok(serde_json::Value::String(content_hash))
        }

        serde_json::Value::Object(obj) => {
            // Build { head => vec![tails] } map
            let mut keep_head_tails = HashMap::new();
            for path in keep_paths.as_ref() {
                let (head, tail_opt) = json_path::split_head_tail(path)?;
                let tails = keep_head_tails.entry(head.to_owned()).or_insert_with(Vec::new);
                if let Some(tail) = tail_opt {
                    tails.push(tail);
                }
            }

            let mut mask_root = true;
            let mut canonical_json_entries = Vec::new();
            let mut keys: Vec<_> = obj.keys().collect();
            keys.sort();
            for key in keys {
                ensure!(
                    *key == normalize_unicode(key),
                    "Data to be digested must contain field names normalized with Unicode NFKD"
                );

                let value = obj.get(key).expect("serde_json keys() impl error");
                if let Some(tails) = keep_head_tails.get(key) {
                    // Found object key present in keep_paths option, skip masking current branch of tree
                    mask_root = false;
                    if tails.is_empty() {
                        // This is the exact Json path to keep open, do not mask anything
                        canonical_json_entries.push((key, value.to_owned()));
                    } else {
                        // This is a partial match for a Json path to keep open, recurse to mask it partially
                        let partial_value = mask_json_subtree(value, tails)?;
                        canonical_json_entries.push((key, partial_value));
                    }
                } else {
                    // This path does not match any paths, mask it fully
                    let fully_masked_value = mask_json_subtree(value, vec![])?;
                    canonical_json_entries.push((key, fully_masked_value));
                };
            }

            if mask_root {
                let canonical_entry_strs = canonical_json_entries
                    .iter()
                    .filter_map(|(key, val)| {
                        let canonical_key =
                            canonical_json(&serde_json::Value::String((*key).to_string())).ok()?;
                        Some(format!("{}:{}", canonical_key, serde_json::to_string(val).ok()?))
                    })
                    .collect::<Vec<_>>();
                ensure!(
                    canonical_entry_strs.len() == canonical_json_entries.len(),
                    "Implementation error: failed to serialize JSON node entries"
                );

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

/// Convenience function to transform a JSON value into a (partially) masked JSON value.
/// Only subtrees matching the provided JSON path pattern will be kept,
/// all other subtrees will be masked.
///
/// Nearly equivalent to `mask_json_subtree(&json_value, split_alternatives(keep_paths_str))`,
/// but always returns a string, not a `serde_json::Value`.
pub fn selective_digest_json(
    json_value: &serde_json::Value, keep_paths_str: &str,
) -> Result<String> {
    let keep_paths_vec = json_path::split_alternatives(keep_paths_str);
    let digest_json = match &json_value {
        serde_json::Value::Object(_obj) => mask_json_subtree(json_value, keep_paths_vec),
        serde_json::Value::Array(_arr) => mask_json_subtree(json_value, keep_paths_vec),
        serde_json::Value::String(_s) => Ok(json_value.to_owned()),
        _ => bail!("Json digest is currently implemented only for composite types"),
    }?;
    match digest_json {
        serde_json::Value::String(digest) => Ok(digest),
        // TODO probably a serde_json::to_string() would be enough and faster
        serde_json::Value::Object(_) => canonical_json(&digest_json),
        _ => bail!("Implementation error: digest should always return a string or object"),
    }
}

/// Convenience function calling [`selective_digest_json`] with arbitrary serializable types.
///
/// [`selective_digest_json`]: ./fn.selective_digest_json.html
pub fn selective_digest_data<T: serde::Serialize>(
    data: &T, keep_paths_str: &str,
) -> Result<String> {
    let json_value = serde_json::to_value(&data)?;
    selective_digest_json(&json_value, keep_paths_str)
}

/// Convenience function calling [`selective_digest_json`] with a JSON string.
///
/// [`selective_digest_json`]: ./fn.selective_digest_json.html
pub fn selective_digest_json_str(json_str: &str, keep_paths_str: &str) -> Result<String> {
    ensure!(
        json_str == normalize_unicode(json_str),
        "Json string to be digested must be normalized with Unicode NFKD"
    );

    let json_value: serde_json::Value = serde_json::from_str(json_str)?;
    selective_digest_json(&json_value, keep_paths_str)
}

const KEEP_NOTHING: &str = "";

/// Convenience function for serializable types to mask the whole JSON tree into a digest, keep nothing.
pub fn digest_data<T: serde::Serialize>(data: &T) -> Result<String> {
    selective_digest_data(data, KEEP_NOTHING)
}

/// Convenience function for JSON strings to mask the whole JSON tree into a digest, keep nothing.
pub fn digest_json_str(json_str: &str) -> Result<String> {
    selective_digest_json_str(json_str, KEEP_NOTHING)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::FromHex;
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
    fn reject_non_nfkd() -> Result<()> {
        let key_nfc = String::from_utf8(Vec::from_hex("c3a16c6f6d")?)?;
        let key_nfkd = String::from_utf8(Vec::from_hex("61cc816c6f6d")?)?;
        assert_eq!(key_nfc, "álom");
        assert_eq!(key_nfkd, "álom");

        let str_nfc = format!("{{\"{}\": 1}}", key_nfc);
        let str_nfkd = format!("{{\"{}\": 1}}", key_nfkd);
        assert_eq!(digest_json_str(&str_nfkd)?, "cjuRab8yOeLzxmFY_fEMC79cW5z9XyihRhaGnTSvMabrA8");
        assert!(digest_json_str(&str_nfc).is_err());

        let json_value_nfc: serde_json::Value = serde_json::from_str(&str_nfc)?;
        let json_value_nfkd: serde_json::Value = serde_json::from_str(&str_nfkd)?;
        assert_eq!(
            selective_digest_json(&json_value_nfkd, "")?,
            "cjuRab8yOeLzxmFY_fEMC79cW5z9XyihRhaGnTSvMabrA8"
        );
        assert!(selective_digest_json(&json_value_nfc, "").is_err());
        Ok(())
    }

    #[test]
    fn digest_string_is_idempotent() {
        let content_id = &r#""cjuzC-XxgzNMwYXtw8aMIAeS2Xjlw1hlSNKTvVtUwPuyYo""#;
        let digest_id = digest_data(content_id).unwrap();
        assert_eq!(content_id, &digest_id);
    }

    #[test]
    fn test_json_digest() -> Result<()> {
        let test_obj = TestData { b: 1, a: 2 };
        {
            let digested = digest_data(&test_obj)?;
            assert_eq!(digested, "cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU");
        }
        {
            let digested = digest_data(&[&test_obj, &test_obj])?;
            assert_eq!(digested, "cjuGkDpb1HL7F8xFKDFVj3felfKZzjrJy92-108uuPixNw");
        }
        {
            let digested =
                digest_data(&(&test_obj, "cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU"))?;
            assert_eq!(digested, "cjuGkDpb1HL7F8xFKDFVj3felfKZzjrJy92-108uuPixNw");
        }
        {
            let digested = digest_data(&[
                "cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU",
                "cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU",
            ])?;
            assert_eq!(digested, "cjuGkDpb1HL7F8xFKDFVj3felfKZzjrJy92-108uuPixNw");
        }
        {
            let x = &test_obj;
            let comp = CompositeTestData { z: Some(x.clone()), y: Some(x.clone()) };
            let digested = digest_data(&comp)?;
            assert_eq!(digested, "cjubdcpA0FfHhD8yEpDzZ8vS5sm7yxkrX_wAJgmke2bWRQ");
        }
        {
            let comp = CompositeTestData {
                z: Some("cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU".to_owned()),
                y: Some("cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU".to_owned()),
            };
            let digested = digest_data(&comp)?;
            assert_eq!(digested, "cjubdcpA0FfHhD8yEpDzZ8vS5sm7yxkrX_wAJgmke2bWRQ");
        }
        Ok(())
    }

    #[test]
    fn test_selective_digesting() -> Result<()> {
        let test_obj = TestData { b: 1, a: 2 };
        let x = &test_obj;
        let composite = CompositeTestData { z: Some(x.clone()), y: Some(x.clone()) };
        let double_complex =
            CompositeTestData { z: Some(composite.clone()), y: Some(composite.clone()) };
        let triple_complex =
            CompositeTestData { z: Some(double_complex.clone()), y: Some(double_complex.clone()) };
        {
            let fully_digested = selective_digest_data(&composite, "")?;
            assert_eq!(fully_digested, "cjubdcpA0FfHhD8yEpDzZ8vS5sm7yxkrX_wAJgmke2bWRQ");
        }
        {
            let keep_y = selective_digest_data(&composite, ".y")?;
            assert_eq!(
                keep_y,
                r#"{"y":{"a":2,"b":1},"z":"cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU"}"#
            );
            let val: serde_json::Value = serde_json::from_str(&keep_y)?;
            assert_eq!(digest_data(&val)?, "cjubdcpA0FfHhD8yEpDzZ8vS5sm7yxkrX_wAJgmke2bWRQ");
        }
        {
            let keep_z = selective_digest_data(&composite, ".z")?;
            assert_eq!(
                keep_z,
                r#"{"y":"cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU","z":{"a":2,"b":1}}"#
            );
            let val: serde_json::Value = serde_json::from_str(&keep_z)?;
            assert_eq!(digest_data(&val)?, "cjubdcpA0FfHhD8yEpDzZ8vS5sm7yxkrX_wAJgmke2bWRQ");
        }
        {
            let digest = digest_data(&double_complex)?;
            assert_eq!(digest, "cjuQLebyl_BJipFLibhWiStDBqK5J4JZq15ehUqybfTTKA");
        }
        {
            let keep_yz = selective_digest_data(&double_complex, ".y.z")?;
            assert_eq!(
                keep_yz,
                r#"{"y":{"y":"cjumTq1s6Tn6xkXolxHj4LmAo7DAb-zoPLhEa1BvpovAFU","z":{"a":2,"b":1}},"z":"cjubdcpA0FfHhD8yEpDzZ8vS5sm7yxkrX_wAJgmke2bWRQ"}"#
            );
            let val: serde_json::Value = serde_json::from_str(&keep_yz)?;
            assert_eq!(digest_data(&val)?, "cjuQLebyl_BJipFLibhWiStDBqK5J4JZq15ehUqybfTTKA");
        }
        {
            let digest = digest_data(&triple_complex)?;
            assert_eq!(digest, "cjuik140L3w7LCi6z1eHt7Qgwr2X65-iy8HA6zqrlUdmVk");
        }
        {
            let keep_yz = selective_digest_data(&triple_complex, ".y.y , .z.z")?;
            assert_eq!(
                keep_yz,
                r#"{"y":{"y":{"y":{"a":2,"b":1},"z":{"a":2,"b":1}},"z":"cjubdcpA0FfHhD8yEpDzZ8vS5sm7yxkrX_wAJgmke2bWRQ"},"z":{"y":"cjubdcpA0FfHhD8yEpDzZ8vS5sm7yxkrX_wAJgmke2bWRQ","z":{"y":{"a":2,"b":1},"z":{"a":2,"b":1}}}}"#
            );
            let val: serde_json::Value = serde_json::from_str(&keep_yz)?;
            assert_eq!(digest_data(&val)?, "cjuik140L3w7LCi6z1eHt7Qgwr2X65-iy8HA6zqrlUdmVk");
        }
        Ok(())
    }
}
