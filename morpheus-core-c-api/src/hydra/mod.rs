use super::*;

use iop_keyvault::Networks;
use iop_morpheus_core::crypto::hd::{
    hydra::{Parameters, Plugin, Private, Public},
    BoundPlugin, Vault,
};

pub struct HydraPlugin {
    plugin: BoundPlugin<Plugin, Public, Private>,
}

fn params(network: *const raw::c_char, account: i32) -> Fallible<Parameters> {
    let network = convert::str_in(network)?;
    let network = Networks::by_name(network)?;
    Ok(Parameters::new(network, account))
}

#[no_mangle]
pub extern "C" fn HydraPlugin_rewind(
    vault: *mut Vault, unlock_pwd: *const raw::c_char, network: *const raw::c_char, account: i32,
) -> CPtrResult<raw::c_void> {
    let vault = unsafe { convert::borrow_mut_in(vault) };
    let mut fun = || {
        let unlock_password = convert::str_in(unlock_pwd)?;
        let params = params(network, account)?;
        Plugin::rewind(vault, unlock_password, &params)?;
        Ok(())
    };
    cresult_void(fun())
}

#[no_mangle]
pub extern "C" fn HydraPlugin_get(
    vault: *mut Vault, network: *const raw::c_char, account: i32,
) -> CPtrResult<HydraPlugin> {
    let vault = unsafe { convert::borrow_mut_in(vault) };
    let fun = || {
        let params = params(network, account)?;
        let plugin = Plugin::get(&vault, &params)?;
        let hydra = HydraPlugin { plugin };
        Ok(convert::move_out(hydra))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn delete_HydraPlugin(hydra: *mut HydraPlugin) {
    if hydra.is_null() {
        return;
    }
    let hydra = unsafe { Box::from_raw(hydra) };
    drop(hydra); // NOTE redundant, but clearer than let _plugin = ...;
}

#[no_mangle]
pub extern "C" fn HydraPlugin_address(
    hydra: *mut HydraPlugin, idx: i32,
) -> CPtrResult<raw::c_char> {
    let hydra = unsafe { convert::borrow_in(hydra) };
    let fun = || {
        let address = hydra.plugin.public()?.key(idx)?;
        let adress_str = address.to_p2pkh_addr();
        Ok(convert::string_out(adress_str))
    };
    cresult(fun())
}
