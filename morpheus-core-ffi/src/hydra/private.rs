use super::*;

#[no_mangle]
pub extern "C" fn HydraPlugin_private(
    hydra: *mut CHydraPlugin, unlock_pwd: *const raw::c_char,
) -> CPtrResult<Private> {
    let hydra = unsafe { convert::borrow_mut_in(hydra) };
    let fun = || {
        let unlock_password = convert::str_in(unlock_pwd)?;
        let private = hydra.plugin.private(unlock_password)?;
        Ok(convert::move_out(private))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn delete_HydraPrivate(private: *mut Private) {
    if private.is_null() {
        return;
    }
    let private = unsafe { Box::from_raw(private) };
    drop(private); // NOTE redundant, but clearer than let _plugin = ...;
}
