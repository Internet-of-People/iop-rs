use super::*;

#[no_mangle]
pub extern "C" fn delete_SecpKeyId(secp_id: *mut SecpKeyId) {
    delete(secp_id)
}

#[no_mangle]
pub extern "C" fn SecpKeyId_from_address(
    address: *mut raw::c_char, network: *mut raw::c_char,
) -> CPtrResult<SecpKeyId> {
    let fun = || {
        let address = unsafe { convert::str_in(address)? };
        let network = unsafe { convert::str_in(network)? };

        let network = Networks::by_name(network)?;
        let id = SecpKeyId::from_p2pkh_addr(address, network)?;

        let id = convert::move_out(id);
        Ok(id)
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn SecpKeyId_to_address(
    secp_id: *const SecpKeyId, network: *const raw::c_char,
) -> CPtrResult<raw::c_char> {
    let secp_id = unsafe { convert::borrow_in(secp_id) };
    let fun = || {
        let network = unsafe { convert::str_in(network)? };

        let network = Networks::by_name(network)?;
        let address = secp_id.to_p2pkh_addr(network.p2pkh_addr());

        let address = convert::string_out(address);
        Ok(address)
    };
    cresult(fun())
}
