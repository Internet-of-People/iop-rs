mod call_context;
mod convert;
mod unsafe_fut;

use std::os::raw;
// use std::panic::catch_unwind; // TODO consider panic unwinding strategies

use failure::{err_msg, Fallible};
use serde_json;

use self::call_context::{CallContext, Callback, RequestId};
use self::convert::RawSlice;
use crate::sdk::SdkContext;
use bip39::MnemonicType;
use iop_morpheus_core::{
    crypto::{json_digest, sign::Nonce},
    data::diddoc::BlockHeight,
};

#[no_mangle]
pub extern "C" fn init_sdk(
    id: *mut RequestId, success: Callback<*mut SdkContext>, error: Callback<*const raw::c_char>,
) -> () {
    let fun = || {
        let sdk = SdkContext::new()?;
        Ok(Box::into_raw(Box::new(sdk)))
    };
    CallContext::new(id, success, error).run(fun)
}

#[no_mangle]
pub extern "C" fn mask_json(
    _sdk: *mut SdkContext, raw_json: *const raw::c_char, raw_keep_paths: *const raw::c_char,
    id: *mut RequestId, success: Callback<*mut raw::c_char>, error: Callback<*const raw::c_char>,
) {
    let fun = || {
        let json_str = convert::str_in(raw_json)?;
        let json_val: serde_json::Value = serde_json::from_str(json_str)?;
        let keep_paths_str = convert::str_in(raw_keep_paths)?;
        let masked_json = json_digest::mask_json(&json_val, keep_paths_str)?;
        Ok(convert::string_out(masked_json))
    };
    CallContext::new(id, success, error).run(fun)
}

fn code_to_lang(lang: *const raw::c_char) -> Fallible<bip39::Language> {
    let lang_code = convert::str_in(lang)?;
    bip39::Language::from_language_code(&lang_code).ok_or_else(|| err_msg("Unknown language"))
}

#[no_mangle]
pub extern "C" fn bip39_generate_phrase(
    _sdk: *mut SdkContext, lang: *const raw::c_char, id: *mut RequestId,
    success: Callback<*mut raw::c_char>, error: Callback<*const raw::c_char>,
) -> () {
    let fun = || {
        let language = code_to_lang(lang)?;
        let mnemonic = bip39::Mnemonic::new(MnemonicType::Words24, language);
        let phrase = mnemonic.into_phrase();
        Ok(convert::string_out(phrase))
    };
    CallContext::new(id, success, error).run(fun)
}

#[no_mangle]
pub extern "C" fn bip39_validate_phrase(
    _sdk: *mut SdkContext, lang: *const raw::c_char, phrase: *const raw::c_char,
    id: *mut RequestId, success: Callback<*const raw::c_void>, error: Callback<*const raw::c_char>,
) -> () {
    let fun = || {
        let phrase = convert::str_in(phrase)?;
        let language = code_to_lang(lang)?;
        bip39::Mnemonic::validate(phrase, language)?;
        Ok(std::ptr::null())
    };
    CallContext::new(id, success, error).run(fun)
}

#[no_mangle]
pub extern "C" fn bip39_list_words(
    _sdk: *mut SdkContext, lang: *const raw::c_char, pref: *const raw::c_char, id: *mut RequestId,
    success: Callback<*mut RawSlice<*mut raw::c_char>>, error: Callback<*const raw::c_char>,
) -> () {
    let fun = || {
        let prefix = convert::str_in(pref)?;
        let language = code_to_lang(lang)?;
        let matching_words = language
            .wordlist()
            .get_words_by_prefix(prefix)
            .iter()
            .map(|word| word.to_string())
            .collect::<Vec<_>>();
        let raw_slice = convert::RawSlice::from(matching_words);
        Ok(Box::into_raw(Box::new(raw_slice)))
    };
    CallContext::new(id, success, error).run(fun)
}

#[no_mangle]
pub extern "C" fn create_vault(
    sdk: *mut SdkContext, seed: *const raw::c_char, path: *const raw::c_char, id: *mut RequestId,
    success: Callback<*const raw::c_void>, error: Callback<*const raw::c_char>,
) -> () {
    let sdk = unsafe { &mut *sdk };
    let fun = || {
        sdk.create_vault(convert::str_in(seed)?, convert::str_in(path)?)?;
        Ok(std::ptr::null())
    };
    CallContext::new(id, success, error).run(fun)
}

#[no_mangle]
pub extern "C" fn load_vault(
    sdk: *mut SdkContext, path: *const raw::c_char, id: *mut RequestId,
    success: Callback<*const raw::c_void>, error: Callback<*const raw::c_char>,
) {
    let sdk = unsafe { &mut *sdk };
    let fun = || {
        sdk.load_vault(convert::str_in(path)?)?;
        Ok(std::ptr::null())
    };
    CallContext::new(id, success, error).run(fun)
}

#[no_mangle]
pub extern "C" fn fake_ledger(
    sdk: *mut SdkContext, id: *mut RequestId, success: Callback<*const raw::c_void>,
    error: Callback<*const raw::c_char>,
) {
    let sdk = unsafe { &mut *sdk };
    let fun = || {
        sdk.fake_ledger()?;
        Ok(std::ptr::null())
    };
    CallContext::new(id, success, error).run(fun)
}

#[no_mangle]
pub extern "C" fn real_ledger(
    sdk: *mut SdkContext, url: *const raw::c_char, id: *mut RequestId,
    success: Callback<*const raw::c_void>, error: Callback<*const raw::c_char>,
) {
    let sdk = unsafe { &mut *sdk };
    let fun = || {
        sdk.real_ledger(convert::str_in(url)?)?;
        Ok(std::ptr::null())
    };
    CallContext::new(id, success, error).run(fun)
}

#[no_mangle]
pub extern "C" fn list_dids(
    sdk: *mut SdkContext, id: *mut RequestId, success: Callback<*mut RawSlice<*mut raw::c_char>>,
    error: Callback<*const raw::c_char>,
) {
    let sdk = unsafe { &mut *sdk };
    let fun = || {
        let did_strs = sdk.list_dids()?.into_iter().map(|did| did.to_string()).collect::<Vec<_>>();
        let raw_slice = RawSlice::from(did_strs);
        Ok(Box::into_raw(Box::new(raw_slice)))
    };
    CallContext::new(id, success, error).run(fun)
}

#[no_mangle]
pub extern "C" fn create_did(
    sdk: *mut SdkContext, id: *mut RequestId, success: Callback<*mut raw::c_char>,
    error: Callback<*const raw::c_char>,
) {
    let sdk = unsafe { &mut *sdk };
    let fun = || {
        let did = sdk.create_did()?;
        Ok(convert::string_out(did.to_string()))
    };
    CallContext::new(id, success, error).run(fun)
}

#[no_mangle]
pub extern "C" fn get_document(
    sdk: *mut SdkContext, did: *const raw::c_char, id: *mut RequestId,
    success: Callback<*mut raw::c_char>, error: Callback<*const raw::c_char>,
) {
    let sdk = unsafe { &mut *sdk };
    let fun = || {
        let did_str = convert::str_in(did)?;
        let did = did_str.parse()?;
        let document = sdk.get_document(&did)?;
        let json = serde_json::to_string(&document)?;
        Ok(convert::string_out(json))
    };
    CallContext::new(id, success, error).run(fun)
}

#[no_mangle]
pub extern "C" fn generate_nonce(
    _sdk: *mut SdkContext, id: *mut RequestId, success: Callback<*mut raw::c_char>,
    error: Callback<*const raw::c_char>,
) {
    let fun = || {
        let nonce = Nonce::new();
        Ok(convert::string_out(nonce.0))
    };
    CallContext::new(id, success, error).run(fun)
}

#[no_mangle]
pub extern "C" fn sign_witness_request(
    sdk: *mut SdkContext, req: *const raw::c_char, auth: *const raw::c_char, id: *mut RequestId,
    success: Callback<*mut raw::c_char>, error: Callback<*const raw::c_char>,
) {
    let sdk = unsafe { &mut *sdk };
    let fun = || {
        let req_str = convert::str_in(req)?;
        let req = serde_json::from_str(req_str)?;
        let auth_str = format!("{:?}", convert::str_in(auth)?);
        let auth = serde_json::from_str(&auth_str)?;
        let signed_request = sdk.sign_witness_request(req, &auth)?;
        let json = serde_json::to_string(&signed_request)?;
        Ok(convert::string_out(json))
    };
    CallContext::new(id, success, error).run(fun)
}

#[no_mangle]
pub extern "C" fn sign_witness_statement(
    sdk: *mut SdkContext, stmnt: *const raw::c_char, auth: *const raw::c_char, id: *mut RequestId,
    success: Callback<*mut raw::c_char>, error: Callback<*const raw::c_char>,
) {
    let sdk = unsafe { &mut *sdk };
    let fun = || {
        let stmnt_str = convert::str_in(stmnt)?;
        let stmnt = serde_json::from_str(stmnt_str)?;
        let auth_str = format!("{:?}", convert::str_in(auth)?);
        let auth = serde_json::from_str(&auth_str)?;
        let signed_stmnt = sdk.sign_witness_statement(stmnt, &auth)?;
        let json = serde_json::to_string(&signed_stmnt)?;
        Ok(convert::string_out(json))
    };
    CallContext::new(id, success, error).run(fun)
}

#[no_mangle]
pub extern "C" fn sign_claim_presentation(
    sdk: *mut SdkContext, pres: *const raw::c_char, auth: *const raw::c_char, id: *mut RequestId,
    success: Callback<*mut raw::c_char>, error: Callback<*const raw::c_char>,
) {
    let sdk = unsafe { &mut *sdk };
    let fun = || {
        let pres_str = convert::str_in(pres)?;
        let presentation = serde_json::from_str(pres_str)?;
        let auth_str = format!("{:?}", convert::str_in(auth)?);
        let auth = serde_json::from_str(&auth_str)?;
        let signed_pres = sdk.sign_claim_presentation(presentation, &auth)?;
        let json = serde_json::to_string(&signed_pres)?;
        Ok(convert::string_out(json))
    };
    CallContext::new(id, success, error).run(fun)
}

// TODO consider type mapping of Right to C and maybe adding list_rights()

#[no_mangle]
pub extern "C" fn has_right_at(
    sdk: *mut SdkContext, did: *const raw::c_char, auth: *const raw::c_char,
    right: *const raw::c_char, height: u64, id: *mut RequestId,
    success: Callback<*mut raw::c_uchar>, error: Callback<*const raw::c_char>,
) {
    let sdk = unsafe { &mut *sdk };
    let fun = || {
        let did_str = convert::str_in(did)?;
        let did = did_str.parse()?;
        let auth_str = convert::str_in(auth)?;
        let auth = auth_str.parse()?;
        let right_str = convert::str_in(right)?;
        let right = right_str.parse()?;
        let document = sdk.get_document(&did)?;
        let height = height as BlockHeight;
        let has_right = document.has_right_at(&auth, right, height)?;
        Ok(convert::bool_out(has_right))
    };
    CallContext::new(id, success, error).run(fun)
}

#[no_mangle]
pub extern "C" fn is_tombstoned_at(
    sdk: *mut SdkContext, did: *const raw::c_char, height: u64, id: *mut RequestId,
    success: Callback<*mut raw::c_uchar>, error: Callback<*const raw::c_char>,
) {
    let sdk = unsafe { &mut *sdk };
    let fun = || {
        let did_str = convert::str_in(did)?;
        let did = did_str.parse()?;
        let document = sdk.get_document(&did)?;
        let height = height as BlockHeight;
        let tombstoned = document.is_tombstoned_at(height)?;
        Ok(convert::bool_out(tombstoned))
    };
    CallContext::new(id, success, error).run(fun)
}

#[no_mangle]
pub extern "C" fn close_sdk(sdk: *mut SdkContext) {
    if sdk.is_null() {
        return;
    }
    let sdk = unsafe { Box::from_raw(sdk) };
    sdk.close().unwrap();
}
