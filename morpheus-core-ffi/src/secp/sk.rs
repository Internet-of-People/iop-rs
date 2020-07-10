use super::*;

#[no_mangle]
pub extern "C" fn delete_SecpPrivateKey(secp_sk: *mut SecpPrivateKey) {
    delete(secp_sk)
}

#[no_mangle]
pub extern "C" fn SecpPrivateKey_from_ark_passphrase(
    passphrase: *mut raw::c_char,
) -> CPtrResult<SecpPrivateKey> {
    let fun = || {
        let passphrase = unsafe { convert::str_in(passphrase) }?;
        let secp_sk = SecpPrivateKey::from_ark_passphrase(passphrase)?;
        Ok(convert::move_out(secp_sk))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn SecpPrivateKey_to_wif(
    secp_sk: *mut SecpPrivateKey, network: *mut raw::c_char,
) -> CPtrResult<raw::c_char> {
    let secp_sk = unsafe { convert::borrow_in(secp_sk) };
    let fun = || {
        let network = unsafe { convert::str_in(network) }?;
        let network = Networks::by_name(network)?;
        let wif = secp_sk.to_wif(network.wif(), Bip178::Compressed);
        Ok(convert::string_out(wif))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn SecpPrivateKey_public_key(secp_sk: *mut SecpPrivateKey) -> *mut SecpPublicKey {
    let secp_sk = unsafe { convert::borrow_in(secp_sk) };
    convert::move_out(secp_sk.public_key())
}

#[no_mangle]
pub extern "C" fn SecpPrivateKey_sign_ecdsa(
    secp_sk: *mut SecpPrivateKey, data: *mut CSlice<u8>,
) -> *mut SecpSignature {
    let secp_sk = unsafe { convert::borrow_in(secp_sk) };
    let data = unsafe { convert::borrow_in(data) };
    let secp_sig = secp_sk.sign(data.as_slice());
    convert::move_out(secp_sig)
}
