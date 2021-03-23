use super::*;

#[wasm_bindgen]
pub fn encrypt(plain_text: &[u8], password: &str) -> Result<Vec<u8>, JsValue> {
    let nonce = keyvault_encrypt::nonce().map_err_to_js()?;
    keyvault_encrypt::encrypt(plain_text, password, &nonce).map_err_to_js()
}

#[wasm_bindgen]
pub fn decrypt(cipher_text: &[u8], password: &str) -> Result<Vec<u8>, JsValue> {
    keyvault_encrypt::decrypt(cipher_text, password).map_err_to_js()
}
