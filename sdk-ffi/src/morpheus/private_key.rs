use super::*;

#[unsafe(no_mangle)]
pub extern "C" fn delete_MorpheusPrivateKey(key: *mut MorpheusPrivateKey) {
    delete(key)
}

#[unsafe(no_mangle)]
pub extern "C" fn MorpheusPrivateKey_bip32_path(key: *mut MorpheusPrivateKey) -> *mut raw::c_char {
    let key = unsafe { convert::borrow_in(key) };
    convert::string_out(key.node().path().to_string())
}

#[unsafe(no_mangle)]
pub extern "C" fn MorpheusPrivateKey_kind(key: *mut MorpheusPrivateKey) -> *mut raw::c_char {
    let key = unsafe { convert::borrow_in(key) };
    convert::string_out(format!("{:?}", key.path().kind()))
}

#[unsafe(no_mangle)]
pub extern "C" fn MorpheusPrivateKey_idx(key: *mut MorpheusPrivateKey) -> *mut i32 {
    let key = unsafe { convert::borrow_in(key) };
    convert::move_out(key.path().idx())
}

#[unsafe(no_mangle)]
pub extern "C" fn MorpheusPrivateKey_neuter(
    key: *mut MorpheusPrivateKey,
) -> *mut MorpheusPublicKey {
    let key = unsafe { convert::borrow_in(key) };
    convert::move_out(key.neuter())
}

#[unsafe(no_mangle)]
pub extern "C" fn MorpheusPrivateKey_private_key(key: *mut MorpheusPrivateKey) -> *mut MPrivateKey {
    let key = unsafe { convert::borrow_in(key) };
    convert::move_out(key.private_key())
}
