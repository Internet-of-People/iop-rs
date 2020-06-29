use super::*;

#[no_mangle]
pub extern "C" fn HydraPlugin_public(hydra: *mut CHydraPlugin) -> CPtrResult<Public> {
    let hydra = unsafe { convert::borrow_in(hydra) };
    let fun = || {
        let public = hydra.plugin.public()?;
        Ok(convert::move_out(public))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn delete_HydraPublic(public: *mut Public) {
    if public.is_null() {
        return;
    }
    let public = unsafe { Box::from_raw(public) };
    drop(public); // NOTE redundant, but clearer than let _plugin = ...;
}

// TODO better fit to Wasm interface, this is still only for experimentation
#[no_mangle]
pub extern "C" fn HydraPublic_address(public: *mut Public, idx: i32) -> CPtrResult<raw::c_char> {
    let fun = || {
        let public = unsafe { convert::borrow_mut_in(public) };
        let address = public.key(idx)?;
        let adress_str = address.to_p2pkh_addr();
        Ok(convert::string_out(adress_str))
    };
    cresult(fun())
}
