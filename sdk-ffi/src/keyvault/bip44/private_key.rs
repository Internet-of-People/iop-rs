use super::*;

#[no_mangle]
pub extern "C" fn delete_Bip44Key(bip44_key: *mut Bip44Key<Secp256k1>) {
    delete(bip44_key)
}

#[no_mangle]
pub extern "C" fn Bip44Key_node(bip44_key: *mut Bip44Key<Secp256k1>) -> *mut Bip32Node<Secp256k1> {
    let bip44_key = unsafe { convert::borrow_in(bip44_key) };
    convert::move_out(bip44_key.node().clone())
}

#[no_mangle]
pub extern "C" fn Bip44Key_private_key(bip44_key: *mut Bip44Key<Secp256k1>) -> *mut SecpPrivateKey {
    let bip44_key = unsafe { convert::borrow_in(bip44_key) };
    convert::move_out(bip44_key.to_private_key())
}

#[no_mangle]
pub extern "C" fn Bip44Key_slip44_get(bip44_key: *mut Bip44Key<Secp256k1>) -> i32 {
    let bip44_key = unsafe { convert::borrow_in(bip44_key) };
    bip44_key.bip44_path().parent().parent().parent().slip44()
}

#[no_mangle]
pub extern "C" fn Bip44Key_account_get(bip44_key: *mut Bip44Key<Secp256k1>) -> i32 {
    let bip44_key = unsafe { convert::borrow_in(bip44_key) };
    bip44_key.bip44_path().parent().parent().account()
}

#[no_mangle]
pub extern "C" fn Bip44Key_change_get(bip44_key: *mut Bip44Key<Secp256k1>) -> bool {
    let bip44_key = unsafe { convert::borrow_in(bip44_key) };
    bip44_key.bip44_path().parent().chain().into()
}

#[no_mangle]
pub extern "C" fn Bip44Key_key_get(bip44_key: *mut Bip44Key<Secp256k1>) -> i32 {
    let bip44_key = unsafe { convert::borrow_in(bip44_key) };
    bip44_key.bip44_path().key()
}

#[no_mangle]
pub extern "C" fn Bip44Key_path_get(bip44_key: *mut Bip44Key<Secp256k1>) -> *mut raw::c_char {
    let bip44_key = unsafe { convert::borrow_in(bip44_key) };
    convert::string_out(bip44_key.bip32_path().to_string())
}

#[no_mangle]
pub extern "C" fn Bip44Key_neuter(
    bip44_key: *mut Bip44Key<Secp256k1>,
) -> CPtrResult<Bip44PublicKey<Secp256k1>> {
    let bip44_key = unsafe { convert::borrow_in(bip44_key) };
    let fun = || {
        let pub_key = bip44_key.neuter();
        Ok(convert::move_out(pub_key))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn Bip44Key_wif_get(bip44_pubkey: *mut Bip44Key<Secp256k1>) -> *mut raw::c_char {
    let bip44_key = unsafe { convert::borrow_in(bip44_pubkey) };
    convert::string_out(bip44_key.to_wif())
}
