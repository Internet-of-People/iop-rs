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

#[no_mangle]
pub extern "C" fn HydraPublic_xpub_get(public: *mut Public) -> CPtrResult<raw::c_char> {
    let public = unsafe { convert::borrow_in(public) };
    let fun = || {
        let xpub = public.xpub()?;
        Ok(convert::string_out(xpub))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn HydraPublic_receive_keys_get(public: *mut Public) -> CPtrResult<u32> {
    let public = unsafe { convert::borrow_in(public) };
    let fun = || {
        let receive_keys = public.receive_keys()?;
        Ok(convert::move_out(receive_keys))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn HydraPublic_change_keys_get(public: *mut Public) -> CPtrResult<u32> {
    let public = unsafe { convert::borrow_in(public) };
    let fun = || {
        let change_keys = public.change_keys()?;
        Ok(convert::move_out(change_keys))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn HydraPublic_key(
    public: *mut Public, idx: i32,
) -> CPtrResult<Bip44PublicKey<Secp256k1>> {
    let fun = || {
        let public = unsafe { convert::borrow_mut_in(public) };
        let pk = public.key(idx)?;
        Ok(convert::move_out(pk))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn HydraPublic_key_by_address(
    public: *mut Public, address: *const raw::c_char,
) -> CPtrResult<Bip44PublicKey<Secp256k1>> {
    let public = unsafe { convert::borrow_in(public) };
    let fun = || {
        let address = unsafe { convert::str_in(address) }?;
        let pk = public.key_by_p2pkh_addr(address)?;
        Ok(convert::move_out(pk))
    };
    cresult(fun())
}
