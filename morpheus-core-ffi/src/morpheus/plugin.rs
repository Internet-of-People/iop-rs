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
pub extern "C" fn delete_MorpheusPlugin(morpheus: *mut CMorpheusPlugin) {
    if morpheus.is_null() {
        return;
    }
    let morpheus = unsafe { Box::from_raw(morpheus) };
    drop(morpheus); // NOTE redundant, but clearer than let _plugin = ...;
}

// TODO Temporary function to test integration.
#[no_mangle]
pub extern "C" fn MorpheusPlugin_persona(
    morpheus: *mut CMorpheusPlugin, idx: i32,
) -> CPtrResult<raw::c_char> {
    let morpheus = unsafe { convert::borrow_in(morpheus) };
    let fun = || {
        let persona = morpheus.plugin.public()?.personas()?.key(idx)?;
        let persona_str = persona.key_id().to_string();
        Ok(convert::string_out(persona_str))
    };
    cresult(fun())
}
