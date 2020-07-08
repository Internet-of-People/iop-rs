use super::*;

#[no_mangle]
pub extern "C" fn delete_SecpPrivateKey(secp_sk: *mut SecpPrivateKey) {
    delete(secp_sk)
}
