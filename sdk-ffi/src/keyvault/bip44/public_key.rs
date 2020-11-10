use super::*;

#[no_mangle]
pub extern "C" fn delete_Bip44PublicKey(bip44_pubkey: *mut Bip44PublicKey<Secp256k1>) {
    delete(bip44_pubkey)
}

#[no_mangle]
pub extern "C" fn Bip44PublicKey_node(
    bip44_key: *mut Bip44PublicKey<Secp256k1>,
) -> *mut Bip32PublicNode<Secp256k1> {
    let bip44_key = unsafe { convert::borrow_in(bip44_key) };
    convert::move_out(bip44_key.node().clone())
}

#[no_mangle]
pub extern "C" fn Bip44PublicKey_public_key(
    bip44_key: *mut Bip44PublicKey<Secp256k1>,
) -> *mut SecpPublicKey {
    let bip44_key = unsafe { convert::borrow_in(bip44_key) };
    convert::move_out(bip44_key.to_public_key())
}

#[no_mangle]
pub extern "C" fn Bip44PublicKey_key_id(
    bip44_key: *mut Bip44PublicKey<Secp256k1>,
) -> *mut SecpKeyId {
    let bip44_key = unsafe { convert::borrow_in(bip44_key) };
    convert::move_out(bip44_key.to_key_id())
}

#[no_mangle]
pub extern "C" fn Bip44PublicKey_slip44_get(bip44_key: *mut Bip44PublicKey<Secp256k1>) -> i32 {
    let bip44_key = unsafe { convert::borrow_in(bip44_key) };
    bip44_key.bip44_path().parent().parent().parent().slip44()
}

#[no_mangle]
pub extern "C" fn Bip44PublicKey_account_get(bip44_key: *mut Bip44PublicKey<Secp256k1>) -> i32 {
    let bip44_key = unsafe { convert::borrow_in(bip44_key) };
    bip44_key.bip44_path().parent().parent().account()
}

#[no_mangle]
pub extern "C" fn Bip44PublicKey_change_get(bip44_key: *mut Bip44PublicKey<Secp256k1>) -> bool {
    let bip44_key = unsafe { convert::borrow_in(bip44_key) };
    bip44_key.bip44_path().parent().chain().into()
}

#[no_mangle]
pub extern "C" fn Bip44PublicKey_key_get(bip44_key: *mut Bip44PublicKey<Secp256k1>) -> i32 {
    let bip44_key = unsafe { convert::borrow_in(bip44_key) };
    bip44_key.bip44_path().key()
}

#[no_mangle]
pub extern "C" fn Bip44PublicKey_path_get(
    bip44_key: *mut Bip44PublicKey<Secp256k1>,
) -> *mut raw::c_char {
    let bip44_key = unsafe { convert::borrow_in(bip44_key) };
    convert::string_out(bip44_key.bip32_path().to_string())
}

#[no_mangle]
pub extern "C" fn Bip44PublicKey_address_get(
    bip44_pubkey: *mut Bip44PublicKey<Secp256k1>,
) -> *mut raw::c_char {
    let bip44_pubkey = unsafe { convert::borrow_in(bip44_pubkey) };
    convert::string_out(bip44_pubkey.to_p2pkh_addr())
}
