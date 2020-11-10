use super::*;

#[no_mangle]
pub extern "C" fn delete_MPublicKey(pk: *mut MPublicKey) {
    delete(pk)
}

#[no_mangle]
pub extern "C" fn MPublicKey_prefix() -> *mut raw::c_char {
    let prefix = MPublicKey::PREFIX.to_string();
    convert::string_out(prefix)
}

#[no_mangle]
// TODO Should _from_secp functions rather take ownership of the input like in Rust?
pub extern "C" fn MPublicKey_from_secp(secp: *mut SecpPublicKey) -> *mut MPublicKey {
    let secp = unsafe { convert::borrow_in(secp) };
    convert::move_out(MPublicKey::from(secp.clone()))
}

#[no_mangle]
pub extern "C" fn MPublicKey_from_bytes(data: *mut CSlice<u8>) -> CPtrResult<MPublicKey> {
    let data = unsafe { convert::borrow_in(data) };
    let fun = || {
        let pk = MPublicKey::from_bytes(data.as_slice())?;
        Ok(convert::move_out(pk))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MPublicKey_from_string(input: *mut raw::c_char) -> CPtrResult<MPublicKey> {
    let fun = || {
        let input = unsafe { convert::str_in(input)? };
        let pk: MPublicKey = input.parse()?;
        Ok(convert::move_out(pk))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MPublicKey_to_bytes(pk: *mut MPublicKey) -> *mut CSlice<u8> {
    let pk = unsafe { convert::borrow_in(pk) };
    convert::move_out(CSlice::from(pk.to_bytes()))
}

#[no_mangle]
pub extern "C" fn MPublicKey_to_string(pk: *mut MPublicKey) -> *mut raw::c_char {
    let pk = unsafe { convert::borrow_in(pk) };
    convert::string_out(pk.to_string())
}

#[no_mangle]
pub extern "C" fn MPublicKey_key_id(pk: *mut MPublicKey) -> *mut MKeyId {
    let pk = unsafe { convert::borrow_in(pk) };
    convert::move_out(pk.key_id())
}

#[no_mangle]
pub extern "C" fn MPublicKey_validate_id(pk: *mut MPublicKey, key_id: *mut MKeyId) -> bool {
    let pk = unsafe { convert::borrow_in(pk) };
    let key_id = unsafe { convert::borrow_in(key_id) };
    pk.validate_id(key_id)
}

#[no_mangle]
pub extern "C" fn MPublicKey_verify(
    pk: *mut MPublicKey, data: *mut CSlice<u8>, sig: *mut MSignature,
) -> bool {
    let pk = unsafe { convert::borrow_in(pk) };
    let data = unsafe { convert::borrow_in(data) };
    let sig = unsafe { convert::borrow_in(sig) };
    pk.verify(data.as_slice(), &sig)
}
