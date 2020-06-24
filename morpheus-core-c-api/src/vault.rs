use super::*;

use iop_morpheus_core::crypto::hd::Vault;

fn is_dirty(vault: &Vault) -> Fallible<bool> {
    let flag_state = vault.to_modifiable();
    let dirty_flag_value = flag_state.try_borrow()?;
    Ok(*dirty_flag_value)
}

fn set_dirty(vault: &Vault, value: bool) -> Fallible<()> {
    let mut vault_state = vault.to_modifiable();
    let mut dirty_flag = vault_state.try_borrow_mut()?;
    *dirty_flag = value;
    Ok(())
}

#[no_mangle]
pub extern "C" fn vault_new(
    seed: *const raw::c_char, word25: *const raw::c_char, unlock_pwd: *const raw::c_char,
    context: *mut CallContext<*mut Vault>,
) {
    let fun = || {
        let seed = convert::str_in(seed)?;
        let bip39_password = convert::str_in(word25)?;
        let unlock_password = convert::str_in(unlock_pwd)?;

        let vault = Vault::create(seed, bip39_password, unlock_password)?;

        // // TODO parameters, especially Testnet must come from the caller
        // let hyd_params = hydra::Parameters::new(&hyd::Testnet, 0);
        // // TODO this should be an asynchronous operation which might change the binding as well
        // hydra::Plugin::rewind(&mut vault, unlock_password, &hyd_params)?;
        // let hydra_plugin = hydra::Plugin::get(&vault, &hyd_params)?;

        Ok(convert::move_out(vault))
    };
    unsafe { convert::borrow_mut_in(context).run(fun) }
}

#[no_mangle]
pub extern "C" fn vault_free(vault: *mut Vault) {
    if vault.is_null() {
        return;
    }
    let vault = unsafe { Box::from_raw(vault) };
    drop(vault); // NOTE redundant, but clearer than let _vault = ...;
}

#[no_mangle]
pub extern "C" fn vault_is_dirty(vault: *mut Vault, context: *mut CallContext<*mut raw::c_uchar>) {
    let vault = unsafe { &*vault };
    let fun = || Ok(convert::bool_out(is_dirty(vault)?));
    unsafe { convert::borrow_mut_in(context).run(fun) }
}

#[no_mangle]
pub extern "C" fn vault_to_json(vault: *mut Vault, context: *mut CallContext<*mut raw::c_char>) {
    let vault = unsafe { &*vault };
    let fun = || {
        let vault_json = serde_json::to_string(&vault)?;
        set_dirty(vault, false)?;
        Ok(convert::string_out(vault_json))
    };
    unsafe { convert::borrow_mut_in(context).run(fun) }
}

#[no_mangle]
pub extern "C" fn json_to_vault(json: *const raw::c_char, context: *mut CallContext<*mut Vault>) {
    let fun = || {
        let json = convert::str_in(json)?;
        let vault = serde_json::from_str(json)?;
        Ok(convert::move_out(vault))
    };
    unsafe { convert::borrow_mut_in(context).run(fun) }
}

// #[no_mangle]
// pub extern "C" fn list_dids(
//     sdk: *mut SdkContext, id: *mut RequestId, success: Callback<*mut RawSlice<*mut raw::c_char>>,
//     error: Callback<*const raw::c_char>,
// ) {
//     let sdk = unsafe { &mut *sdk };
//     let fun = || {
//         let did_strs = sdk.list_dids()?.into_iter().map(|did| did.to_string()).collect::<Vec<_>>();
//         let raw_slice = RawSlice::from(did_strs);
//         Ok(convert::move_out(raw_slice))
//     };
//     unsafe { convert::borrow_mut_in(context).run(fun) }
// }
//
// #[no_mangle]
// pub extern "C" fn create_did(
//     sdk: *mut SdkContext, id: *mut RequestId, success: Callback<*mut raw::c_char>,
//     error: Callback<*const raw::c_char>,
// ) {
//     let sdk = unsafe { &mut *sdk };
//     let fun = || {
//         let did = sdk.create_did()?;
//         Ok(convert::string_out(did.to_string()))
//     };
//     unsafe { convert::borrow_mut_in(context).run(fun) }
// }
//
// #[no_mangle]
// pub extern "C" fn sign_witness_request(
//     sdk: *mut SdkContext, req: *const raw::c_char, auth: *const raw::c_char, id: *mut RequestId,
//     success: Callback<*mut raw::c_char>, error: Callback<*const raw::c_char>,
// ) {
//     let sdk = unsafe { &mut *sdk };
//     let fun = || {
//         let req_str = convert::str_in(req)?;
//         let req = serde_json::from_str(req_str)?;
//         let auth_str = format!("{:?}", convert::str_in(auth)?);
//         let auth = serde_json::from_str(&auth_str)?;
//         let signed_request = sdk.sign_witness_request(req, &auth)?;
//         let json = serde_json::to_string(&signed_request)?;
//         Ok(convert::string_out(json))
//     };
//     unsafe { convert::borrow_mut_in(context).run(fun) }
// }
//
// #[no_mangle]
// pub extern "C" fn sign_witness_statement(
//     sdk: *mut SdkContext, stmnt: *const raw::c_char, auth: *const raw::c_char, id: *mut RequestId,
//     success: Callback<*mut raw::c_char>, error: Callback<*const raw::c_char>,
// ) {
//     let sdk = unsafe { &mut *sdk };
//     let fun = || {
//         let stmnt_str = convert::str_in(stmnt)?;
//         let stmnt = serde_json::from_str(stmnt_str)?;
//         let auth_str = format!("{:?}", convert::str_in(auth)?);
//         let auth = serde_json::from_str(&auth_str)?;
//         let signed_stmnt = sdk.sign_witness_statement(stmnt, &auth)?;
//         let json = serde_json::to_string(&signed_stmnt)?;
//         Ok(convert::string_out(json))
//     };
//     unsafe { convert::borrow_mut_in(context).run(fun) }
// }
//
// #[no_mangle]
// pub extern "C" fn sign_claim_presentation(
//     sdk: *mut SdkContext, pres: *const raw::c_char, auth: *const raw::c_char, id: *mut RequestId,
//     success: Callback<*mut raw::c_char>, error: Callback<*const raw::c_char>,
// ) {
//     let sdk = unsafe { &mut *sdk };
//     let fun = || {
//         let pres_str = convert::str_in(pres)?;
//         let presentation = serde_json::from_str(pres_str)?;
//         let auth_str = format!("{:?}", convert::str_in(auth)?);
//         let auth = serde_json::from_str(&auth_str)?;
//         let signed_pres = sdk.sign_claim_presentation(presentation, &auth)?;
//         let json = serde_json::to_string(&signed_pres)?;
//         Ok(convert::string_out(json))
//     };
//     unsafe { convert::borrow_mut_in(context).run(fun) }
// }
