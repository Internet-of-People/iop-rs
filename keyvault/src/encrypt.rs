//! A thin integration of Argon2i and XChaCha20Poly1305 algorithms from the orion crate to encrypt/decrypt in-memory blobs with a password.

use failure::{err_msg, Fallible};
use orion::aead::SecretKey;

fn password_to_key(pw: &str, salt: &[u8]) -> Fallible<SecretKey> {
    use orion::hazardous::stream::chacha20::CHACHA_KEYSIZE;
    use orion::kdf::{derive_key, Password, Salt};

    let pw = Password::from_slice(pw.as_bytes()).map_err(|_| err_msg("Password is too short"))?;
    let salt = Salt::from_slice(salt).map_err(|_| err_msg("Salt is too short"))?;
    let kdf_key = derive_key(&pw, &salt, 15, 1024, CHACHA_KEYSIZE as u32)
        .map_err(|_| err_msg("Could not derive key from password"))?;
    let key = SecretKey::from_slice(kdf_key.unprotected_as_bytes())
        .map_err(|_| err_msg("Could not convert key"))?;
    Ok(key)
}

/// Generates a 24-byte random nonce that can be used with the [`encrypt`] function.
///
/// # Errors
///
/// When the underlying platform is unable to provide enough random entropy.
pub fn nonce() -> Fallible<[u8; 24]> {
    let mut result = [0u8; 24];
    getrandom::getrandom(&mut result)?;
    Ok(result)
}

/// Encrypts the plaintext with a password. Make sure the password is not weak. Make sure to generate an exactly 24-byte random nonce for each call
/// otherwise there is a chance of weakening the key if the same nonce is used more than once. The ciphertext returned will be 40 bytes longer than the
/// plaintext.
pub fn encrypt(
    plaintext: impl AsRef<[u8]>, pw: impl AsRef<str>, nonce: impl AsRef<[u8]>,
) -> Fallible<Vec<u8>> {
    use orion::hazardous::{
        aead::xchacha20poly1305::{seal, Nonce, SecretKey as XSecretKey},
        mac::poly1305::POLY1305_OUTSIZE,
        stream::xchacha20::XCHACHA_NONCESIZE,
    };
    let plaintext = plaintext.as_ref();
    let pw = pw.as_ref();
    let nonce = nonce.as_ref();

    let key = password_to_key(pw, nonce)?;
    let key = XSecretKey::from_slice(key.unprotected_as_bytes())
        .map_err(|_| err_msg("Key is invalid"))?;

    let nonce = Nonce::from_slice(nonce).map_err(|_| err_msg("Nonce is too short"))?;

    let out_len = match plaintext.len().checked_add(XCHACHA_NONCESIZE + POLY1305_OUTSIZE) {
        Some(min_out_len) => min_out_len,
        None => return Err(err_msg("Plaintext is too long")),
    };
    let mut output = vec![0u8; out_len];
    output[..XCHACHA_NONCESIZE].copy_from_slice(nonce.as_ref());

    seal(&key, &nonce, plaintext, None, &mut output[XCHACHA_NONCESIZE..])
        .map_err(|_| err_msg("Could not convert key"))?;

    Ok(output)
}

/// Decrypts the ciphertext with a password. The format of the ciphertext is defined by the [`encrypt`] function. Only the matching password will
/// decrypt the ciphertext.
pub fn decrypt(ciphertext: impl AsRef<[u8]>, pw: impl AsRef<str>) -> Fallible<Vec<u8>> {
    use orion::aead::open;
    use orion::hazardous::stream::xchacha20::XCHACHA_NONCESIZE;

    let ciphertext = ciphertext.as_ref();
    let pw = pw.as_ref();

    if ciphertext.len() <= (XCHACHA_NONCESIZE) {
        return Err(err_msg("Ciphertext is too short"));
    }

    let key = password_to_key(pw, &ciphertext[..XCHACHA_NONCESIZE])?;
    open(&key, ciphertext).map_err(|_| err_msg("Ciphertext was tampered with"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encrypt_decrypt_roundtrip() -> Fallible<()> {
        let nonce = nonce()?;

        let password = "password123";
        let message = "Be at the big tree at 5pm tomorrow!";
        let plaintext = message.as_bytes().to_owned();
        let ciphertext = encrypt(&plaintext, password, &nonce)?;
        let plaintext2 = decrypt(&ciphertext, password)?;

        assert_eq!(&plaintext2, &plaintext);
        assert_eq!(message.len(), 35);
        assert_eq!(ciphertext.len(), 35 + 40);

        Ok(())
    }
}
