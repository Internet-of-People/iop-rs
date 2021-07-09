use super::*;

#[no_mangle]
pub extern "C" fn delete_SecpPublicKey(secp_pk: *mut SecpPublicKey) {
    delete(secp_pk)
}

#[no_mangle]
pub extern "C" fn SecpPublicKey_fromString(hex: *const raw::c_char) -> CPtrResult<SecpPublicKey> {
    let fun = || {
        let hex = unsafe { convert::str_in(hex) }?;
        let secp_pk = SecpPublicKey::from_str(hex)?;
        Ok(convert::move_out(secp_pk))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn SecpPublicKey_to_string(
    secp_pk: *const SecpPublicKey,
) -> CPtrResult<raw::c_char> {
    let secp_pk = unsafe { convert::borrow_in(secp_pk) };
    let fun = || {
        let hex = secp_pk.to_string();
        Ok(convert::string_out(hex))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn SecpPublicKey_key_id(secp_pk: *const SecpPublicKey) -> *mut SecpKeyId {
    let secp_pk = unsafe { convert::borrow_in(secp_pk) };
    convert::move_out(secp_pk.key_id())
}

#[no_mangle]
pub extern "C" fn SecpPublicKey_ark_key_id(secp_pk: *const SecpPublicKey) -> *mut SecpKeyId {
    let secp_pk = unsafe { convert::borrow_in(secp_pk) };
    convert::move_out(secp_pk.ark_key_id())
}

#[no_mangle]
pub extern "C" fn SecpPublicKey_validate_id(
    secp_pk: *mut SecpPublicKey, secp_id: *mut SecpKeyId,
) -> bool {
    let secp_pk = unsafe { convert::borrow_in(secp_pk) };
    let secp_id = unsafe { convert::borrow_in(secp_id) };
    secp_pk.validate_id(secp_id)
}

#[no_mangle]
pub extern "C" fn SecpPublicKey_validate_ark_id(
    secp_pk: *mut SecpPublicKey, secp_id: *mut SecpKeyId,
) -> bool {
    let secp_pk = unsafe { convert::borrow_in(secp_pk) };
    let secp_id = unsafe { convert::borrow_in(secp_id) };
    secp_pk.validate_ark_id(secp_id)
}

#[no_mangle]
pub extern "C" fn SecpPublicKey_validate_ecdsa(
    secp_pk: *mut SecpPublicKey, data: *mut CSlice<u8>, secp_sig: *mut SecpSignature,
) -> bool {
    let secp_pk = unsafe { convert::borrow_in(secp_pk) };
    let data = unsafe { convert::borrow_in(data) };
    let secp_sig = unsafe { convert::borrow_in(secp_sig) };
    secp_pk.verify(data.as_slice(), secp_sig)
}
