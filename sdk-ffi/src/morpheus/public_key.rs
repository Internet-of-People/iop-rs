use super::*;

#[unsafe(no_mangle)]
pub extern "C" fn delete_MorpheusPublicKey(key: *mut MorpheusPublicKey) {
    delete(key)
}

#[unsafe(no_mangle)]
pub extern "C" fn MorpheusPublicKey_bip32_path(key: *mut MorpheusPublicKey) -> *mut raw::c_char {
    let key = unsafe { convert::borrow_in(key) };
    convert::string_out(key.node().path().to_string())
}

#[unsafe(no_mangle)]
pub extern "C" fn MorpheusPublicKey_kind(key: *mut MorpheusPublicKey) -> *mut raw::c_char {
    let key = unsafe { convert::borrow_in(key) };
    convert::string_out(format!("{:?}", key.path().kind()))
}

#[unsafe(no_mangle)]
pub extern "C" fn MorpheusPublicKey_idx(key: *mut MorpheusPublicKey) -> *mut i32 {
    let key = unsafe { convert::borrow_in(key) };
    convert::move_out(key.path().idx())
}

#[unsafe(no_mangle)]
pub extern "C" fn MorpheusPublicKey_public_key(key: *mut MorpheusPublicKey) -> *mut MPublicKey {
    let key = unsafe { convert::borrow_in(key) };
    convert::move_out(key.public_key())
}
