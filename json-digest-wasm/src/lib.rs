use wasm_bindgen::prelude::*;

use json_digest::*;

pub fn err_to_js<E: ToString>(e: E) -> JsValue {
    JsValue::from(e.to_string())
}

pub trait MapJsError<T> {
    fn map_err_to_js(self) -> Result<T, JsValue>;
}

impl<T, E: ToString> MapJsError<T> for Result<T, E> {
    fn map_err_to_js(self) -> Result<T, JsValue> {
        self.map_err(err_to_js)
    }
}

#[wasm_bindgen(js_name = selectiveDigestJson)]
pub fn selective_digest(data: &JsValue, keep_properties_list: &str) -> Result<String, JsValue> {
    let serde_data: serde_json::Value = data.into_serde().map_err_to_js()?;
    let digested_data_str =
        selective_digest_json(&serde_data, keep_properties_list).map_err_to_js()?;
    Ok(digested_data_str)
}

#[wasm_bindgen(js_name = digestJson)]
pub fn digest(data: &JsValue) -> Result<String, JsValue> {
    selective_digest(data, "")
}

#[wasm_bindgen(js_name = stringifyJson)]
pub fn stringify(data: &JsValue) -> Result<String, JsValue> {
    let serde_data: serde_json::Value = data.into_serde().map_err_to_js()?;
    let stringified = canonical_json(&serde_data).map_err_to_js()?;
    Ok(stringified)
}
