use super::*;

#[no_mangle]
pub extern "C" fn delete_Did(did: *mut Did) {
    delete(did)
}

#[no_mangle]
pub extern "C" fn Did_prefix() -> *mut raw::c_char {
    let prefix = Did::PREFIX.to_string();
    convert::string_out(prefix)
}

#[no_mangle]
pub extern "C" fn Did_from_string(input: *const raw::c_char) -> CPtrResult<Did> {
    let fun = || {
        let input = unsafe { convert::str_in(input)? };
        let did: Did = input.parse()?;
        Ok(convert::move_out(did))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn Did_from_key_id(id: *const MKeyId) -> *mut Did {
    let id = unsafe { convert::borrow_in(id) };
    convert::move_out(Did::from(id.clone()))
}

#[no_mangle]
pub extern "C" fn Did_to_string(did: *const Did) -> *mut raw::c_char {
    let did = unsafe { convert::borrow_in(did) };
    convert::string_out(did.to_string())
}

#[no_mangle]
pub extern "C" fn Did_default_key_id(did: *const Did) -> *mut MKeyId {
    let did = unsafe { convert::borrow_in(did) };
    convert::move_out(did.default_key_id())
}
