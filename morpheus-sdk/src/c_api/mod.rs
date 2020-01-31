// TODO provide a C API that allows
// 1. selecting a DID from a vault
// 2. selecting a key for a DID
// 3. sign content with the selected key
// +1 maybe later: create a witness request

mod call_context;
mod convert;
mod sdk;
mod unsafe_fut;

use std::os::raw;
// use std::panic::catch_unwind; // TODO consider panic unwinding strategies

use serde_json;

use call_context::{CallContext, Callback, RequestId};
use convert::RawSlice;
use sdk::SdkContext;

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

//#[no_mangle]
//pub extern "C" fn ping(
//    sdk: *mut SdkContext, message: *const raw::c_char, delay_secs: u32, id: *mut RequestId,
//    success: Callback<*mut raw::c_char>, error: Callback<*const raw::c_char>,
//) -> () {
//    let sdk = unsafe { &mut *sdk };
//    let fut = async move {
//        let message = convert::str_in(message)?;
//        tokio::time::delay_for(std::time::Duration::from_secs(delay_secs.into())).await;
//        if message.starts_with("fail") {
//            failure::bail!(message);
//        }
//        let out = format!(
//            "From Rust: You sent '{}'. It works even with async operations involved.",
//            message
//        );
//        Ok(convert::string_out(out))
//    };
//    CallContext::new(id, success, error).run_async(sdk.reactor(), fut)
//}

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
        let did_vec = sdk.list_dids()?;
        let cptr_box_slice =
            did_vec.iter().map(|did| convert::string_out(did.to_string())).collect::<Box<[_]>>();
        let raw_box_slice = Box::into_raw(cptr_box_slice);
        let raw_slice: RawSlice<*mut raw::c_char> = unsafe { &mut *raw_box_slice }.into();
        //unsafe { Box::from_raw(raw_box_slice) };
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
pub extern "C" fn sign_witness_request(
    sdk: *mut SdkContext, req: *const raw::c_char, auth: *const raw::c_char, id: *mut RequestId,
    success: Callback<*mut raw::c_char>, error: Callback<*const raw::c_char>,
) {
    let sdk = unsafe { &mut *sdk };
    let fun = || {
        let req_str = convert::str_in(req)?;
        //let req = req_str.parse()?;
        let auth_str = format!("{:?}", convert::str_in(auth)?);
        let auth = serde_json::from_str(&auth_str)?;
        let signed_request = sdk.sign_witness_request(req_str.to_owned(), &auth)?;
        let json = serde_json::to_string(&signed_request)?;
        Ok(convert::string_out(json))
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
