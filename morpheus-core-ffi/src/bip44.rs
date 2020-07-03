use super::*;

#[no_mangle]
pub extern "C" fn Bip44Key_privateKey_get(
    bip44_key: *mut Bip44Key<Secp256k1>,
) -> CPtrResult<SecpPrivateKey> {
    let bip44_key = unsafe { convert::borrow_in(bip44_key) };
    let fun = || {
        let pub_key = bip44_key.to_private_key();
        Ok(convert::move_out(pub_key))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn delete_Bip44Key(bip44_key: *mut Bip44Key<Secp256k1>) {
    if bip44_key.is_null() {
        return;
    }
    let bip44_key = unsafe { Box::from_raw(bip44_key) };
    drop(bip44_key); // NOTE redundant, but clearer than let _plugin = ...;
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
pub extern "C" fn Bip44PublicKey_publicKey_get(
    bip44_key: *mut Bip44PublicKey<Secp256k1>,
) -> CPtrResult<SecpPublicKey> {
    let bip44_key = unsafe { convert::borrow_in(bip44_key) };
    let fun = || {
        let pub_key = bip44_key.to_public_key();
        Ok(convert::move_out(pub_key))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn delete_Bip44PublicKey(bip44_pubkey: *mut Bip44PublicKey<Secp256k1>) {
    if bip44_pubkey.is_null() {
        return;
    }
    let bip44_pubkey = unsafe { Box::from_raw(bip44_pubkey) };
    drop(bip44_pubkey); // NOTE redundant, but clearer than let _plugin = ...;
}

#[no_mangle]
pub extern "C" fn Bip44PublicKey_address_get(
    bip44_pubkey: *mut Bip44PublicKey<Secp256k1>,
) -> CPtrResult<raw::c_char> {
    let bip44_pubkey = unsafe { convert::borrow_in(bip44_pubkey) };
    let fun = || {
        let key_str = bip44_pubkey.to_p2pkh_addr();
        Ok(convert::string_out(key_str))
    };
    cresult(fun())
}
