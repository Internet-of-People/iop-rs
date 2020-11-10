use super::*;

#[no_mangle]
pub extern "C" fn delete_JwtParser(parser: *mut JwtParser) {
    delete(parser)
}

#[no_mangle]
pub extern "C" fn JwtParser_new(
    token: *const raw::c_char, current_time: *const i64,
) -> CPtrResult<JwtParser> {
    let fun = || {
        let token = unsafe { convert::str_in(token)? };
        let current_time =
            unsafe { convert::borrow_in_opt(current_time) }.map(|secs| Utc.timestamp(*secs, 0));
        let parser = JwtParser::new(token, current_time)?;
        Ok(convert::move_out(parser))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn JwtParser_public_key_get(parser: *const JwtParser) -> *mut MPublicKey {
    let parser = unsafe { convert::borrow_in(parser) };
    convert::move_out(parser.public_key())
}

#[no_mangle]
pub extern "C" fn JwtParser_created_at_get(parser: *const JwtParser) -> i64 {
    let parser = unsafe { convert::borrow_in(parser) };
    parser.created_at().timestamp()
}

#[no_mangle]
pub extern "C" fn JwtParser_time_to_live_get(parser: *const JwtParser) -> i64 {
    let parser = unsafe { convert::borrow_in(parser) };
    parser.time_to_live().num_seconds()
}

#[no_mangle]
pub extern "C" fn JwtParser_content_id_get(parser: *const JwtParser) -> *mut raw::c_char {
    let parser = unsafe { convert::borrow_in(parser) };
    let content_id: Option<String> = parser.content_id().cloned();
    convert::string_out_opt(content_id)
}
