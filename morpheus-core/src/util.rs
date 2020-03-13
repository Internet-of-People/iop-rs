use failure::{bail, Fallible};

// Path pattern syntax is based on JQ patterns, see https://stedolan.github.io/jq/manual/#Basicfilters
pub(crate) fn match_json(tree: &serde_json::Value, paths_pattern: &str) -> Fallible<bool> {
    for path in json_path_alternatives(paths_pattern) {
        if match_json_path(tree, path)? {
            return Ok(true);
        }
    }
    Ok(false)
}

// Assumes no whitespaces and no alternate paths in parameter
pub(crate) fn match_json_path(tree: &serde_json::Value, path: &str) -> Fallible<bool> {
    match tree {
        serde_json::Value::Object(map) => {
            let (property_name, path_tail_opt) = json_path_head_tail(path)?;
            let path_tail = match path_tail_opt {
                None => return Ok(map.contains_key(property_name)),
                Some(path_tail) => path_tail,
            };

            let property_val_opt = map.get(property_name);
            match property_val_opt {
                Some(subtree) => match_json_path(subtree, path_tail),
                None => Ok(false),
            }
        }
        // TODO should we support arrays?
        // serde_json::Value::Array(arr) => {???},
        _ => Ok(false),
    }
}

/// ```
/// use morpheus_core::util::json_path_alternatives;
/// assert_eq!( json_path_alternatives(".a , .b.c , .d"), vec![".a", ".b.c", ".d"]);
/// ```
pub fn json_path_alternatives(paths_pattern: &str) -> Vec<&str> {
    paths_pattern
        .split_terminator(',') // split alternative tree paths (enabling trailing comma)
        .map(|item| item.trim()) // trim all items to enable whitespaces near commas
        .collect()
}

/// ```
/// use morpheus_core::util::json_path_head_tail;
/// assert_eq!(json_path_head_tail(".a").unwrap(), ("a", None));
/// assert_eq!(json_path_head_tail(".a.b.c").unwrap(), ("a", Some(".b.c")));
/// ```
pub fn json_path_head_tail(path: &str) -> Fallible<(&str, Option<&str>)> {
    if !path.starts_with('.') {
        bail!("Path must start with '.' but it's: {}", path);
    }
    let path = &path[1..];

    let dot_idx_opt = path.find('.');
    let tuple = match dot_idx_opt {
        None => (path, None),
        Some(dot_idx) => {
            let (path_head, path_tail) = path.split_at(dot_idx);
            (path_head, Some(path_tail))
        }
    };
    Ok(tuple)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn sample_json_object() -> serde_json::Value {
        json!({
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+42 1234567",
                "+44 2345678"
            ],
            "address": {
                "country": "Germany",
                "city": "Berlin",
                "zip": 1234,
                "street": {
                    "name": "Some Street",
                    "number": "1"
                }
            }
        })
    }

    #[test]
    fn json_path_matches() -> Fallible<()> {
        let obj = sample_json_object();

        assert!(match_json(&obj, "invalidpath").is_err());
        assert_eq!(match_json(&obj, ".notpresent").ok(), Some(false));
        assert_eq!(match_json(&obj, ".name").ok(), Some(true));
        assert_eq!(match_json(&obj, ".a").ok(), Some(false));
        assert_eq!(match_json(&obj, ".age").ok(), Some(true));
        assert_eq!(match_json(&obj, ".ageover").ok(), Some(false));
        assert_eq!(match_json(&obj, ".phones").ok(), Some(true));
        assert_eq!(match_json(&obj, ".phones.first").ok(), Some(false));
        assert_eq!(match_json(&obj, ".address").ok(), Some(true));
        assert_eq!(match_json(&obj, ".address.country").ok(), Some(true));
        assert_eq!(match_json(&obj, ".address.city").ok(), Some(true));
        assert_eq!(match_json(&obj, ".address.street").ok(), Some(true));
        assert_eq!(match_json(&obj, ".address.street.name").ok(), Some(true));
        assert_eq!(match_json(&obj, ".address.street.number").ok(), Some(true));
        assert_eq!(match_json(&obj, ".address.street.none").ok(), Some(false));
        assert_eq!(match_json(&obj, ".address.phone").ok(), Some(false));

        assert!(match_json(&obj, ".none , invalid , .ageover ").is_err());
        assert_eq!(match_json(&obj, ".none , .fake , .ageover ").ok(), Some(false));
        assert_eq!(match_json(&obj, ".none , .fake , .ageover , .age").ok(), Some(true));
        assert_eq!(match_json(&obj, ".addr , .address.street.num, .xxx").ok(), Some(false));
        assert_eq!(match_json(&obj, ".addr , .address.street.number, .xxx").ok(), Some(true));
        assert_eq!(match_json(&obj, ".ad , .address.street.numbers, .xxx").ok(), Some(false));

        Ok(())
    }

    #[test]
    fn selective_json_masking() -> Fallible<()> {
        let _obj = sample_json_object();
        // TODO
        Ok(())
    }
}
