use super::*;

use iop_keyvault::PublicKey;
use iop_morpheus_core::crypto::hd::{
    morpheus::{Plugin, Private, Public},
    BoundPlugin, Vault,
};

pub struct Morpheus {
    plugin: BoundPlugin<Plugin, Public, Private>,
}

impl Morpheus {
    fn new(vault: &mut Vault, unlock_password: &str) -> Fallible<Self> {
        Plugin::rewind(vault, unlock_password)?;
        let plugin = Plugin::get(&vault)?;
        Ok(Self { plugin })
    }
}

#[no_mangle]
pub unsafe extern "C" fn vault_morpheus(
    vault: *mut Vault, unlock_pwd: *const raw::c_char, context: *mut CallContext<*mut Morpheus>,
) {
    let vault = convert::borrow_mut_in(vault);
    let fun = || {
        let unlock_password = convert::str_in(unlock_pwd)?;
        let morpheus = Morpheus::new(vault, unlock_password)?;
        Ok(convert::move_out(morpheus))
    };
    convert::borrow_mut_in(context).run(fun)
}

#[no_mangle]
pub extern "C" fn morpheus_free(morpheus: *mut Morpheus) {
    if morpheus.is_null() {
        return;
    }
    let morpheus = unsafe { Box::from_raw(morpheus) };
    drop(morpheus); // NOTE redundant, but clearer than let _plugin = ...;
}

#[no_mangle]
pub extern "C" fn morpheus_persona(
    morpheus: *mut Morpheus, idx: i32, context: *mut CallContext<*mut raw::c_char>,
) {
    let morpheus = unsafe { convert::borrow_in(morpheus) };
    let fun = || {
        let persona = morpheus.plugin.public()?.personas()?.key(idx)?;
        let persona_str = persona.key_id().to_string();
        Ok(convert::string_out(persona_str))
    };
    unsafe { convert::borrow_mut_in(context).run(fun) }
}

// #[no_mangle]
// pub extern "C" fn morpheus_personas(
//     morpheus: *mut Morpheus, context: *mut CallContext<*mut RawSlice<*mut raw::c_char>>,
// ) {
//     let morpheus = unsafe { convert::borrow_in(morpheus) };
//     let fun = || {
//         let personas = morpheus.plugin.public()?.personas()?;
//         let persona_strs = personas.into_iter().map(|did| did.to_string()).collect::<Vec<_>>();
//         let raw_slice = RawSlice::from(persona_strs);
//         Ok(convert::move_out(raw_slice))
//     };
//     unsafe { convert::borrow_mut_in(context).run(fun) }
// }

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
