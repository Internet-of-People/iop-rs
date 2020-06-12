use iop_keyvault::encrypt as e;
use wasm_bindgen::prelude::*;

use super::*;

#[wasm_bindgen]
pub fn encrypt(plain_text: &[u8], password: &str, nonce: &[u8]) -> Result<Vec<u8>, JsValue> {
    e::encrypt(plain_text, password, nonce).map_err(err_to_js)
}

#[wasm_bindgen]
pub fn decrypt(cipher_text: &[u8], password: &str) -> Result<Vec<u8>, JsValue> {
    e::decrypt(cipher_text, password).map_err(err_to_js)
}
