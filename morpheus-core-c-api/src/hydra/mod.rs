use super::*;

use iop_keyvault::Networks;
use iop_morpheus_core::crypto::hd::{
    hydra::{Parameters, Plugin, Private, Public},
    BoundPlugin, Vault,
};

pub struct Hydra {
    plugin: BoundPlugin<Plugin, Public, Private>,
}

impl Hydra {
    pub(crate) fn new(
        vault: &mut Vault, unlock_password: &str, network: &str, account: i32,
    ) -> Fallible<Self> {
        let network = Networks::by_name(network)?;
        let params = Parameters::new(network, account);
        Plugin::rewind(vault, unlock_password, &params)?;
        let plugin = Plugin::get(&vault, &params)?;
        Ok(Self { plugin })
    }
}

#[no_mangle]
pub extern "C" fn Vault_hydra(
    vault: *mut Vault, unlock_pwd: *const raw::c_char, network: *const raw::c_char, account: i32,
    context: *mut CallContext<*mut Hydra>,
) {
    let vault = unsafe { convert::borrow_mut_in(vault) };
    let fun = || {
        let unlock_password = convert::str_in(unlock_pwd)?;
        let network = convert::str_in(network)?;
        let plugin = Hydra::new(vault, unlock_password, network, account)?;
        Ok(convert::move_out(plugin))
    };
    unsafe { convert::borrow_mut_in(context).run(fun) }
}

#[no_mangle]
pub extern "C" fn delete_Hydra(hydra: *mut Hydra) {
    if hydra.is_null() {
        return;
    }
    let hydra = unsafe { Box::from_raw(hydra) };
    drop(hydra); // NOTE redundant, but clearer than let _plugin = ...;
}

#[no_mangle]
pub extern "C" fn Hydra_address(
    hydra: *mut Hydra, idx: i32, context: *mut CallContext<*mut raw::c_char>,
) {
    let hydra = unsafe { convert::borrow_in(hydra) };
    let fun = || {
        let address = hydra.plugin.public()?.key(idx)?;
        let adress_str = address.to_p2pkh_addr();
        Ok(convert::string_out(adress_str))
    };
    unsafe { convert::borrow_mut_in(context).run(fun) }
}
