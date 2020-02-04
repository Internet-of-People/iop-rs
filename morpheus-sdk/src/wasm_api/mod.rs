use wasm_bindgen::prelude::*;

//fn err_to_js<E: ToString>(e: E) -> JsValue {
//    JsValue::from(e.to_string())
//}
//
//#[wasm_bindgen]
//pub struct Sdk {
//    inner: keyvault::multicipher::MKeyId,
//}
//
//#[wasm_bindgen]
//impl KeyId {
//    #[wasm_bindgen(constructor)]
//    pub fn new(key_id_str: &str) -> Result<KeyId, JsValue> {
//        let inner: did::ProfileId = key_id_str.parse().map_err(err_to_js)?;
//        Ok(Self { inner })
//    }
//
//    #[wasm_bindgen(js_name = toString)]
//    pub fn to_string(&self) -> String {
//        self.inner.to_string()
//    }
//}
