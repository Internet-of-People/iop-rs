use super::*;

use iop_keyvault::PublicKey;
use iop_morpheus_core::crypto::hd::{
    morpheus::{Plugin, Private, Public},
    BoundPlugin, Vault,
};

pub struct MorpheusPlugin {
    plugin: BoundPlugin<Plugin, Public, Private>,
}

#[no_mangle]
pub extern "C" fn MorpheusPlugin_rewind(
    vault: *mut Vault,
    unlock_pwd: *const raw::c_char,
) -> CPtrResult<raw::c_void> {
    let vault = unsafe { convert::borrow_mut_in(vault) };
    let mut fun = || {
        let unlock_password = convert::str_in(unlock_pwd)?;
        Plugin::rewind(vault, unlock_password)?;
        Ok(())
    };
    fun().into()
}

#[no_mangle]
pub extern "C" fn MorpheusPlugin_get(
    vault: *mut Vault,
) -> CPtrResult<MorpheusPlugin> {
    let vault = unsafe { convert::borrow_mut_in(vault) };
    let fun = || {
        let plugin = Plugin::get(vault)?;
        let morpheus = MorpheusPlugin { plugin };
        Ok(convert::move_out(morpheus))
    };
    fun().into()
}

#[no_mangle]
pub extern "C" fn delete_MorpheusPlugin(morpheus: *mut MorpheusPlugin) {
    if morpheus.is_null() {
        return;
    }
    let morpheus = unsafe { Box::from_raw(morpheus) };
    drop(morpheus); // NOTE redundant, but clearer than let _plugin = ...;
}

#[no_mangle]
pub extern "C" fn MorpheusPlugin_persona(
    morpheus: *mut MorpheusPlugin,
    idx: i32,
) -> CPtrResult<raw::c_char> {
    let morpheus = unsafe { convert::borrow_in(morpheus) };
    let fun = || {
        let persona = morpheus.plugin.public()?.personas()?.key(idx)?;
        let persona_str = persona.key_id().to_string();
        Ok(convert::string_out(persona_str))
    };
    fun().into()
}
