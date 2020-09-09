use super::*;

use iop_keyvault::Networks;
use iop_morpheus_core::crypto::hd::{hydra::Parameters, BoundPlugin, Vault};

pub struct CHydraPlugin {
    pub(crate) plugin: BoundPlugin<Plugin, Public, Private>,
}

fn params(network: *const raw::c_char, account: i32) -> Result<Parameters> {
    let network = unsafe { convert::str_in(network)? };
    let network = Networks::by_name(network)?;
    Ok(Parameters::new(network, account))
}

#[no_mangle]
pub extern "C" fn HydraPlugin_rewind(
    vault: *mut Vault, unlock_pwd: *const raw::c_char, network: *const raw::c_char, account: i32,
) -> CPtrResult<raw::c_void> {
    let vault = unsafe { convert::borrow_mut_in(vault) };
    let mut fun = || {
        let unlock_password = unsafe { convert::str_in(unlock_pwd)? };
        let params = params(network, account)?;
        Plugin::rewind(vault, unlock_password, &params)?;
        Ok(())
    };
    cresult_void(fun())
}

#[no_mangle]
pub extern "C" fn HydraPlugin_get(
    vault: *mut Vault, network: *const raw::c_char, account: i32,
) -> CPtrResult<CHydraPlugin> {
    let vault = unsafe { convert::borrow_mut_in(vault) };
    let fun = || {
        let params = params(network, account)?;
        let plugin = Plugin::get(&vault, &params)?;
        let hydra = CHydraPlugin { plugin };
        Ok(convert::move_out(hydra))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn HydraPlugin_private(
    hydra: *mut CHydraPlugin, unlock_pwd: *const raw::c_char,
) -> CPtrResult<Private> {
    let hydra = unsafe { convert::borrow_in(hydra) };
    let fun = || {
        let unlock_password = unsafe { convert::str_in(unlock_pwd)? };
        let private = hydra.plugin.private(unlock_password)?;
        Ok(convert::move_out(private))
    };
    cresult(fun())
}

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
pub extern "C" fn delete_HydraPlugin(hydra: *mut CHydraPlugin) {
    delete(hydra)
}
