use super::*;

/// Encrypts the plaintext with a password. Make sure the password is not weak.
/// A random nonce is generated for each call so each time the same plaintext is
/// encrypted with the same password, the result is a different ciphertext. The
/// ciphertext returned will be 40 bytes longer than the plaintext.
///
/// @see decrypt
#[wasm_bindgen]
pub fn encrypt(plain_text: &[u8], password: &str) -> Result<Vec<u8>, JsValue> {
    let nonce = keyvault_encrypt::nonce().map_err_to_js()?;
    keyvault_encrypt::encrypt(plain_text, password, &nonce).map_err_to_js()
}

/// Decrypts the ciphertext with a password. The format of the ciphertext is
/// defined by the {@link encrypt} function. Only the matching password will decrypt
/// the ciphertext.
#[wasm_bindgen]
pub fn decrypt(cipher_text: &[u8], password: &str) -> Result<Vec<u8>, JsValue> {
    keyvault_encrypt::decrypt(cipher_text, password).map_err_to_js()
}
