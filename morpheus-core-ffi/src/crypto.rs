use super::*;

use iop_morpheus_core::crypto::sign::Nonce;

#[no_mangle]
pub extern "C" fn selective_digest_json(
    raw_json: *const raw::c_char, raw_keep_paths: *const raw::c_char,
) -> CPtrResult<raw::c_char> {
    let fun = || {
        let json_str = unsafe { convert::str_in(raw_json)? };
        let keep_paths_str = unsafe { convert::str_in(raw_keep_paths)? };
        let digested_json = selective_digest_json_str(json_str, keep_paths_str)?;
        Ok(convert::string_out(digested_json))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn digest_json(raw_json: *const raw::c_char) -> CPtrResult<raw::c_char> {
    let fun = || {
        let json_str = unsafe { convert::str_in(raw_json)? };
        let digest = digest_json_str(json_str)?;
        Ok(convert::string_out(digest))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn stringify_json(raw_json: *const raw::c_char) -> CPtrResult<raw::c_char> {
    let fun = || {
        let json_str = unsafe { convert::str_in(raw_json)? };
        let json_val: serde_json::Value = serde_json::from_str(json_str)?;
        let digested_json = canonical_json(&json_val)?;
        Ok(convert::string_out(digested_json))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn nonce264() -> CPtrResult<raw::c_char> {
    let fun = || {
        let nonce = Nonce::generate();
        Ok(convert::string_out(nonce.0))
    };
    cresult(fun())
}
