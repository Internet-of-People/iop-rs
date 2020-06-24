use super::*;

use iop_keyvault::{Network, Networks};
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

// #[no_mangle]
// pub extern "C" fn vault_hydra(vault: *mut Vault, context: *mut CallContext<*mut Hydra>) {
//     let fun = || {
//         let hyd_params = hydra::Parameters::new(&hyd::Testnet, 0);
//         hydra::Plugin::rewind(&mut vault, unlock_password, &hyd_params)?;
//         // let hydra_plugin = hydra::Plugin::get(&vault, &hyd_params)?;
//         Ok(convert::move_out(vault))
//     };
//     unsafe { convert::borrow_mut_in(context).run(fun) }
// }

#[no_mangle]
pub extern "C" fn hydra_free(hydra: *mut Hydra) {
    if hydra.is_null() {
        return;
    }
    let hydra = unsafe { Box::from_raw(hydra) };
    drop(hydra); // NOTE redundant, but clearer than let _plugin = ...;
}
