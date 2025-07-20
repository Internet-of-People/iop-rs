use super::*;

#[unsafe(no_mangle)]
pub extern "C" fn delete_MPrivateKey(sk: *mut MPrivateKey) {
    delete(sk)
}

#[unsafe(no_mangle)]
// TODO Should _from_secp functions rather take ownership of the input
pub extern "C" fn MPrivateKey_from_secp(secp: *mut SecpPrivateKey) -> *mut MPrivateKey {
    let secp = unsafe { convert::borrow_in(secp) };
    convert::move_out(MPrivateKey::from(secp.clone()))
}

#[unsafe(no_mangle)]
pub extern "C" fn MPrivateKey_public_key(sk: *mut MPrivateKey) -> *mut MPublicKey {
    let sk = unsafe { convert::borrow_in(sk) };
    convert::move_out(sk.public_key())
}

#[unsafe(no_mangle)]
pub extern "C" fn MPrivateKey_sign(sk: *mut MPrivateKey, data: *mut CSlice<u8>) -> *mut MSignature {
    let sk = unsafe { convert::borrow_in(sk) };
    let data = unsafe { convert::borrow_in(data) };
    let sig = sk.sign(data.as_slice());
    convert::move_out(sig)
}
