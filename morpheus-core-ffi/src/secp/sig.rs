use super::*;

#[no_mangle]
pub extern "C" fn delete_SecpSignature(secp_sig: *mut SecpSignature) {
    delete(secp_sig)
}
