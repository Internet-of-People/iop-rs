use super::*;

#[no_mangle]
pub extern "C" fn delete_SecpKeyId(secp_id: *mut SecpKeyId) {
    delete(secp_id)
}
