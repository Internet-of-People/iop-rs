#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

//! This library provides some algorithms to calculate cryptographically secure digests of JSON documents.
//! Since JSON is an ambiguous serialization format, we also had to define a canonical deterministic subset
//! of all allowed documents. Order of keys in an object and Unicode normalization are well-defined in this
//! subset, making it suitable for hashing.

use wasm_bindgen::prelude::*;

use json_digest::*;

/// Converts any error that can be converted into a string into a JavaScript error string
/// usable by wasm_bindgen
pub fn err_to_js<E: ToString>(e: E) -> JsValue {
    JsValue::from(e.to_string())
}

/// An extension trait on [`Result`] that helps easy conversion of Rust errors to JavaScript
/// error strings usable by wasm_bindgen
pub trait MapJsError<T> {
    /// An extension method on [`Result`] to easily convert Rust errors to JavaScript ones.
    ///
    /// ```ignore
    /// #[wasm_bindgen]
    /// pub fn method(&self) -> Result<JsSomething, JsValue> {
    ///     let result: JsSomething = self.fallible().map_err_to_js()?;
    ///     Ok(result)
    /// }
    /// ```
    fn map_err_to_js(self) -> Result<T, JsValue>;
}

impl<T, E: ToString> MapJsError<T> for Result<T, E> {
    fn map_err_to_js(self) -> Result<T, JsValue> {
        self.map_err(err_to_js)
    }
}

/// Returns a canonical string representation of a JSON document, in which any sub-objects not explicitly listed in the
/// second argument are collapsed to their digest. The format of the second argument is inspired by
/// [JQ basic filters](https://stedolan.github.io/jq/manual/#Basicfilters) and these are some examples:
///
/// ```json
/// {
///     "a": {
///         "1": "apple",
///         "2": "banana"
///     },
///     "b": ["some", "array", 0xf, "values"],
///     "c": 42
/// }
/// ```
///
/// - "" -> Same as calling {@link digestJson}
/// - ".a" -> Keep property "a" untouched, the rest will be replaced with their digest. Note that most likely the scalar number "c"
///   does not have enough entropy to avoid a brute-force attack for its digest.
/// - ".b, .c" -> Keeps both properties "b" and "c" unaltered, but "a" will be replaced with the digest of that sub-object.
///
/// You should protect scalar values and easy-to-guess lists by replacing them with an object that has an extra "nonce" property, which
/// has enough entropy. @see wrapJsonWithNonce
#[wasm_bindgen(js_name = selectiveDigestJson)]
pub fn selective_digest(data: &JsValue, keep_properties_list: &str) -> Result<String, JsValue> {
    let serde_data: serde_json::Value = data.into_serde().map_err_to_js()?;
    let digested_data_str =
        selective_digest_json(&serde_data, keep_properties_list).map_err_to_js()?;
    Ok(digested_data_str)
}

/// Calculates the digest of a JSON document. Since this digest is calculated by recursively replacing sub-objects with their digest,
/// it is possible to selectively reveal parts of the document using {@link selectiveDigestJson}
#[wasm_bindgen(js_name = digestJson)]
pub fn digest(data: &JsValue) -> Result<String, JsValue> {
    selective_digest(data, "")
}

/// This function provides a canonical string for any JSON document. Order of the keys in objects, whitespace
/// and unicode normalization are all taken care of, so document that belongs to a single digest is not malleable.
///
/// This is a drop-in replacement for `JSON.stringify(data)`
#[wasm_bindgen(js_name = stringifyJson)]
pub fn stringify(data: &JsValue) -> Result<String, JsValue> {
    let serde_data: serde_json::Value = data.into_serde().map_err_to_js()?;
    let stringified = canonical_json(&serde_data).map_err_to_js()?;
    Ok(stringified)
}

/// You should protect scalar values and easy-to-guess lists by replacing them with an object that has an extra "nonce" property, which
/// has enough entropy. List of all countries, cities in a country, streets in a city are all easy to enumerate for a brute-fore
/// attack.
///
/// For example if you have a string that is a country, you can call this function like `wrapJsonWithNonce("Germany")` and get an
/// object like the following:
///
/// ```json
/// {
///     "nonce": "ukhFsI4a6vIZEDUOBRxJmLroPEQ8FQCjJwbI-Z7bEocGo",
///     "value": "Germany"
/// }
/// ```
#[wasm_bindgen(js_name = wrapWithNonce)]
pub fn wrap_with_nonce(data: &JsValue) -> Result<JsValue, JsValue> {
    let serde_data: serde_json::Value = data.into_serde().map_err_to_js()?;
    let nonce = Nonce264::generate();
    let wrapped_serde = serde_json::json!({
        "nonce": nonce.0,
        "value": serde_data,
    });
    let wrapped_json = JsValue::from_serde(&wrapped_serde).map_err_to_js()?;
    Ok(wrapped_json)
}
