use super::*;

#[no_mangle]
pub extern "C" fn delete_MorpheusPrivateKey(key: *mut MorpheusPrivateKey) {
    if key.is_null() {
        return;
    }
    let key = unsafe { Box::from_raw(key) };
    drop(key); // NOTE redundant, but clearer than let _key = ...;
}

#[no_mangle]
pub extern "C" fn MorpheusPrivateKey_bip32_path(key: *mut MorpheusPrivateKey) -> *mut raw::c_char {
    let key = unsafe { convert::borrow_in(key) };
    convert::string_out(key.node().path().to_string())
}

#[no_mangle]
pub extern "C" fn MorpheusPrivateKey_kind(key: *mut MorpheusPrivateKey) -> *mut raw::c_char {
    let key = unsafe { convert::borrow_in(key) };
    convert::string_out(format!("{:?}", key.path().kind()))
}

#[no_mangle]
pub extern "C" fn MorpheusPrivateKey_idx(key: *mut MorpheusPrivateKey) -> *mut i32 {
    let key = unsafe { convert::borrow_in(key) };
    convert::move_out(key.path().idx())
}

#[no_mangle]
pub extern "C" fn MorpheusPrivateKey_neuter(
    key: *mut MorpheusPrivateKey,
) -> *mut MorpheusPublicKey {
    let key = unsafe { convert::borrow_in(key) };
    convert::move_out(key.neuter())
}

#[no_mangle]
pub extern "C" fn MorpheusPrivateKey_private_key(key: *mut MorpheusPrivateKey) -> *mut MPrivateKey {
    let key = unsafe { convert::borrow_in(key) };
    convert::move_out(key.private_key())
}
