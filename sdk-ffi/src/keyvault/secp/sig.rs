use super::*;

#[unsafe(no_mangle)]
pub extern "C" fn delete_SecpSignature(secp_sig: *mut SecpSignature) {
    delete(secp_sig)
}

#[unsafe(no_mangle)]
pub extern "C" fn SecpSignature_from_der(data: *mut CSlice<u8>) -> CPtrResult<SecpSignature> {
    let data = unsafe { convert::borrow_in(data) };
    let fun = || {
        let secp_sig = SecpSignature::from_der(data.as_slice())?;
        Ok(convert::move_out(secp_sig))
    };
    cresult(fun())
}

#[unsafe(no_mangle)]
pub extern "C" fn SecpSignature_to_der(secp_sig: *mut SecpSignature) -> *mut CSlice<u8> {
    let secp_sig = unsafe { convert::borrow_in(secp_sig) };
    convert::move_out(CSlice::from(secp_sig.to_der()))
}
