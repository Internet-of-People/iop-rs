use super::*;

#[no_mangle]
pub extern "C" fn delete_MSignature(sig: *mut MSignature) {
    if sig.is_null() {
        return;
    }
    let sig = unsafe { Box::from_raw(sig) };
    drop(sig); // NOTE redundant, but clearer than let _sig = ...;
}

#[no_mangle]
pub extern "C" fn MSignature_prefix() -> *mut raw::c_char {
    let prefix = MSignature::PREFIX.to_string();
    convert::string_out(prefix)
}

#[no_mangle]
// TODO Should _from_secp functions rather take ownership of the input
pub extern "C" fn MSignature_from_secp(secp: *mut SecpSignature) -> *mut MSignature {
    let secp = unsafe { convert::borrow_in(secp) };
    convert::move_out(MSignature::from(secp.clone()))
}

#[no_mangle]
pub extern "C" fn MSignature_from_bytes(data: *mut CSlice<u8>) -> CPtrResult<MSignature> {
    let data = unsafe { convert::borrow_in(data) };
    let fun = || {
        let sig = MSignature::from_bytes(data.as_slice())?;
        Ok(convert::move_out(sig))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MSignature_from_string(input: *mut raw::c_char) -> CPtrResult<MSignature> {
    let fun = || {
        let input = unsafe { convert::str_in(input)? };
        let sig: MSignature = input.parse()?;
        Ok(convert::move_out(sig))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MSignature_to_bytes(sig: *mut MSignature) -> *mut CSlice<u8> {
    let sig = unsafe { convert::borrow_in(sig) };
    convert::move_out(CSlice::from(sig.to_bytes()))
}

#[no_mangle]
pub extern "C" fn MSignature_to_string(sig: *mut MSignature) -> *mut raw::c_char {
    let sig = unsafe { convert::borrow_in(sig) };
    convert::string_out(sig.to_string())
}
