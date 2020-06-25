// use super::*;
//
// use iop_morpheus_core::crypto::hd::Vault;
//
// fn is_dirty(vault: &Vault) -> Fallible<bool> {
//     let flag_state = vault.to_modifiable();
//     let dirty_flag_value = flag_state.try_borrow()?;
//     Ok(*dirty_flag_value)
// }
//
// fn set_dirty(vault: &Vault, value: bool) -> Fallible<()> {
//     let mut vault_state = vault.to_modifiable();
//     let mut dirty_flag = vault_state.try_borrow_mut()?;
//     *dirty_flag = value;
//     Ok(())
// }
//
// #[no_mangle]
// pub extern "C" fn Vault_create(
//     seed: *const raw::c_char, word25: *const raw::c_char, unlock_pwd: *const raw::c_char,
//     context: *mut CPtrResult<*mut Vault>,
// ) {
//     let fun = || {
//         let seed = convert::str_in(seed)?;
//         let bip39_password = convert::str_in(word25)?;
//         let unlock_password = convert::str_in(unlock_pwd)?;
//         let vault = Vault::create(seed, bip39_password, unlock_password)?;
//         Ok(convert::move_out(vault))
//     };
//     unsafe { convert::borrow_mut_in(context).run(fun) }
// }
//
// #[no_mangle]
// pub extern "C" fn delete_Vault(vault: *mut Vault) {
//     if vault.is_null() {
//         return;
//     }
//     let vault = unsafe { Box::from_raw(vault) };
//     drop(vault); // NOTE redundant, but clearer than let _vault = ...;
// }
//
// #[no_mangle]
// pub extern "C" fn Vault_is_dirty(vault: *mut Vault, context: *mut CPtrResult<*mut raw::c_uchar>) {
//     let vault = unsafe { convert::borrow_in(vault) };
//     let fun = || Ok(convert::bool_out(is_dirty(vault)?));
//     unsafe { convert::borrow_mut_in(context).run(fun) }
// }
//
// #[no_mangle]
// pub extern "C" fn Vault_save(vault: *mut Vault, context: *mut CPtrResult<*mut raw::c_char>) {
//     let vault = unsafe { &*vault };
//     let fun = || {
//         let vault_json = serde_json::to_string(&vault)?;
//         set_dirty(vault, false)?;
//         Ok(convert::string_out(vault_json))
//     };
//     unsafe { convert::borrow_mut_in(context).run(fun) }
// }
//
// #[no_mangle]
// pub extern "C" fn Vault_load(json: *const raw::c_char, context: *mut CPtrResult<*mut Vault>) {
//     let fun = || {
//         let json = convert::str_in(json)?;
//         let vault = serde_json::from_str(json)?;
//         Ok(convert::move_out(vault))
//     };
//     unsafe { convert::borrow_mut_in(context).run(fun) }
// }
