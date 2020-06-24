use super::*;

#[wasm_bindgen(js_name = mask)]
pub fn mask(data: &JsValue, keep_properties_list: &str) -> Result<String, JsValue> {
    let serde_data: serde_json::Value = data.into_serde().map_err_to_js()?;
    let masked_data_str = mask_json_value(serde_data, keep_properties_list).map_err_to_js()?;
    Ok(masked_data_str)
}

#[wasm_bindgen(js_name = digest)]
pub fn digest(data: &JsValue) -> Result<String, JsValue> {
    mask(data, "")
}
