use super::*;

pub struct CMorpheusPlugin {
    pub(crate) plugin: BoundPlugin<Plugin, Public, Private>,
}

#[no_mangle]
pub extern "C" fn MorpheusPlugin_rewind(
    vault: *mut Vault, unlock_pwd: *const raw::c_char,
) -> CPtrResult<raw::c_void> {
    let vault = unsafe { convert::borrow_mut_in(vault) };
    let mut fun = || {
        let unlock_password = unsafe { convert::str_in(unlock_pwd)? };
        Plugin::rewind(vault, unlock_password)?;
        Ok(())
    };
    cresult_void(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusPlugin_get(vault: *mut Vault) -> CPtrResult<CMorpheusPlugin> {
    let vault = unsafe { convert::borrow_mut_in(vault) };
    let fun = || {
        let plugin = Plugin::get(vault)?;
        let morpheus = CMorpheusPlugin { plugin };
        Ok(convert::move_out(morpheus))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusPlugin_public_get(morpheus: *mut CMorpheusPlugin) -> CPtrResult<Public> {
    let morpheus = unsafe { convert::borrow_in(morpheus) };
    let fun = || {
        let public = morpheus.plugin.public()?;
        Ok(convert::move_out(public))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusPlugin_private(
    morpheus: *mut CMorpheusPlugin, unlock_pwd: *const raw::c_char,
) -> CPtrResult<Private> {
    let morpheus = unsafe { convert::borrow_in(morpheus) };
    let fun = || {
        let unlock_password = unsafe { convert::str_in(unlock_pwd)? };
        let private = morpheus.plugin.private(unlock_password)?;
        Ok(convert::move_out(private))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn delete_MorpheusPlugin(morpheus: *mut CMorpheusPlugin) {
    delete(morpheus)
}
