use super::*;

#[unsafe(no_mangle)]
pub extern "C" fn delete_MKeyId(id: *mut MKeyId) {
    delete(id)
}

#[unsafe(no_mangle)]
pub extern "C" fn MKeyId_prefix() -> *mut raw::c_char {
    let prefix = MKeyId::PREFIX.to_string();
    convert::string_out(prefix)
}

#[unsafe(no_mangle)]
// TODO Should _from_secp functions rather take ownership of the input like in Rust?
pub extern "C" fn MKeyId_from_secp(secp: *mut SecpKeyId) -> *mut MKeyId {
    let secp = unsafe { convert::borrow_in(secp) };
    convert::move_out(MKeyId::from(secp.clone()))
}

#[unsafe(no_mangle)]
pub extern "C" fn MKeyId_from_bytes(data: *mut CSlice<u8>) -> CPtrResult<MKeyId> {
    let data = unsafe { convert::borrow_in(data) };
    let fun = || {
        let id = MKeyId::from_bytes(data.as_slice())?;
        Ok(convert::move_out(id))
    };
    cresult(fun())
}

#[unsafe(no_mangle)]
pub extern "C" fn MKeyId_from_string(input: *mut raw::c_char) -> CPtrResult<MKeyId> {
    let fun = || {
        let input = unsafe { convert::str_in(input)? };
        let id: MKeyId = input.parse()?;
        Ok(convert::move_out(id))
    };
    cresult(fun())
}

#[unsafe(no_mangle)]
pub extern "C" fn MKeyId_to_bytes(id: *mut MKeyId) -> *mut CSlice<u8> {
    let id = unsafe { convert::borrow_in(id) };
    convert::move_out(CSlice::from(id.to_bytes()))
}

#[unsafe(no_mangle)]
pub extern "C" fn MKeyId_to_string(id: *mut MKeyId) -> *mut raw::c_char {
    let id = unsafe { convert::borrow_in(id) };
    convert::string_out(id.to_string())
}
