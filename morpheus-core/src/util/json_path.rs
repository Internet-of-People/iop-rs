use failure::{bail, Fallible};

// Path pattern syntax is based on JQ patterns, see https://stedolan.github.io/jq/manual/#Basicfilters
pub fn matches(tree: &serde_json::Value, paths_pattern: &str) -> Fallible<bool> {
    for single_alternative in split_alternatives(paths_pattern) {
        if match_single(tree, single_alternative)? {
            return Ok(true);
        }
    }
    Ok(false)
}

// Assumes no whitespaces and no alternate paths in parameter
pub fn match_single(tree: &serde_json::Value, path: &str) -> Fallible<bool> {
    match tree {
        serde_json::Value::Object(map) => {
            let (property_name, path_tail_opt) = split_head_tail(path)?;
            let path_tail = match path_tail_opt {
                None => return Ok(map.contains_key(property_name)),
                Some(path_tail) => path_tail,
            };

            let property_val_opt = map.get(property_name);
            match property_val_opt {
                Some(subtree) => match_single(subtree, path_tail),
                None => Ok(false),
            }
        }
        // TODO should we support arrays?
        // serde_json::Value::Array(arr) => {???},
        _ => Ok(false),
    }
}

/// ```
/// use morpheus_core::util::json_path::split_alternatives;
/// assert_eq!( split_alternatives(".a , .b.c , .d"), vec![".a", ".b.c", ".d"]);
/// assert_eq!( split_alternatives(""), Vec::<&str>::new());
/// ```
pub fn split_alternatives(paths_pattern: &str) -> Vec<&str> {
    paths_pattern
        .split_terminator(',') // split alternative tree paths (enabling trailing comma)
        .map(|item| item.trim()) // trim all items to enable whitespaces near commas
        .collect()
}

/// ```
/// use morpheus_core::util::json_path::split_head_tail;
/// assert_eq!(split_head_tail(".a").unwrap(), ("a", None));
/// assert_eq!(split_head_tail(".a.b.c").unwrap(), ("a", Some(".b.c")));
/// ```
pub fn split_head_tail(path: &str) -> Fallible<(&str, Option<&str>)> {
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
    fn path_matches() -> Fallible<()> {
        let obj = sample_json_object();

        assert!(matches(&obj, "invalidpath").is_err());
        assert_eq!(matches(&obj, ".notpresent").ok(), Some(false));
        assert_eq!(matches(&obj, ".name").ok(), Some(true));
        assert_eq!(matches(&obj, ".a").ok(), Some(false));
        assert_eq!(matches(&obj, ".age").ok(), Some(true));
        assert_eq!(matches(&obj, ".ageover").ok(), Some(false));
        assert_eq!(matches(&obj, ".phones").ok(), Some(true));
        assert_eq!(matches(&obj, ".phones.first").ok(), Some(false));
        assert_eq!(matches(&obj, ".address").ok(), Some(true));
        assert_eq!(matches(&obj, ".address.country").ok(), Some(true));
        assert_eq!(matches(&obj, ".address.city").ok(), Some(true));
        assert_eq!(matches(&obj, ".address.street").ok(), Some(true));
        assert_eq!(matches(&obj, ".address.street.name").ok(), Some(true));
        assert_eq!(matches(&obj, ".address.street.number").ok(), Some(true));
        assert_eq!(matches(&obj, ".address.street.none").ok(), Some(false));
        assert_eq!(matches(&obj, ".address.phone").ok(), Some(false));

        assert!(matches(&obj, ".none , invalid , .ageover ").is_err());
        assert_eq!(matches(&obj, ".none , .fake , .ageover ").ok(), Some(false));
        assert_eq!(matches(&obj, ".none , .fake , .ageover , .age").ok(), Some(true));
        assert_eq!(matches(&obj, ".addr , .address.street.num, .xxx").ok(), Some(false));
        assert_eq!(matches(&obj, ".addr , .address.street.number, .xxx").ok(), Some(true));
        assert_eq!(matches(&obj, ".ad , .address.street.numbers, .xxx").ok(), Some(false));

        Ok(())
    }
}
