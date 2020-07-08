use super::*;

#[no_mangle]
pub extern "C" fn SecpPublicKey_fromString(pk_str: *mut raw::c_char) -> CPtrResult<SecpPublicKey> {
    let fun = || {
        let pk_str = unsafe { convert::str_in(pk_str) }?;
        let pk = SecpPublicKey::from_str(pk_str)?;
        Ok(convert::move_out(pk))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn delete_SecpPublicKey(secp_pubkey: *mut SecpPublicKey) {
    delete(secp_pubkey)
}

#[no_mangle]
pub extern "C" fn SecpPublicKey_to_string(pk: *mut SecpPublicKey) -> CPtrResult<raw::c_char> {
    let pub_key = unsafe { convert::borrow_in(pk) };
    let fun = || {
        let key_str = pub_key.to_string();
        Ok(convert::string_out(key_str))
    };
    cresult(fun())
}
