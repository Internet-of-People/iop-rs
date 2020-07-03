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
    if secp_pubkey.is_null() {
        return;
    }
    let secp_pubkey = unsafe { Box::from_raw(secp_pubkey) };
    drop(secp_pubkey); // NOTE redundant, but clearer than let _plugin = ...;
}

#[no_mangle]
pub extern "C" fn SecpPublicKey_toString(pk: *mut SecpPublicKey) -> CPtrResult<raw::c_char> {
    let pub_key = unsafe { convert::borrow_in(pk) };
    let fun = || {
        let key_str = pub_key.to_string();
        Ok(convert::string_out(key_str))
    };
    cresult(fun())
}
