use super::*;

#[no_mangle]
pub extern "C" fn json_mask(
    raw_json: *const raw::c_char, raw_keep_paths: *const raw::c_char,
    context: *mut CallContext<*mut raw::c_char>,
) {
    let fun = || {
        let json_str = convert::str_in(raw_json)?;
        let json_val: serde_json::Value = serde_json::from_str(json_str)?;
        let keep_paths_str = convert::str_in(raw_keep_paths)?;
        let digested_json = json_digest::selective_digest_json(json_val, keep_paths_str)?;
        Ok(convert::string_out(digested_json))
    };
    unsafe { convert::borrow_mut_in(context).run(fun) }
}

#[no_mangle]
pub extern "C" fn nonce_new(context: *mut CallContext<*mut raw::c_char>) {
    let fun = || {
        let nonce = Nonce::generate();
        Ok(convert::string_out(nonce.0))
    };
    unsafe { convert::borrow_mut_in(context).run(fun) }
}
