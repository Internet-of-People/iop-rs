use super::*;

#[unsafe(no_mangle)]
pub extern "C" fn delete_JwtBuilder(builder: *mut JwtBuilder) {
    delete(builder)
}

#[unsafe(no_mangle)]
pub extern "C" fn JwtBuilder_default() -> *mut JwtBuilder {
    convert::move_out(JwtBuilder::default())
}

#[unsafe(no_mangle)]
pub extern "C" fn JwtBuilder_with_content_id(
    content_id: *const raw::c_char,
) -> CPtrResult<JwtBuilder> {
    let fun = || {
        let content_id = unsafe { convert::str_in(content_id)? };
        let builder = JwtBuilder::with_content_id(content_id.to_owned());
        Ok(convert::move_out(builder))
    };
    cresult(fun())
}

#[unsafe(no_mangle)]
pub extern "C" fn JwtBuilder_sign(
    builder: *const JwtBuilder, sk: *const MPrivateKey,
) -> CPtrResult<raw::c_char> {
    let fun = || {
        let builder = unsafe { convert::borrow_in(builder) };
        let sk = unsafe { convert::borrow_in(sk) };
        let token = builder.sign(sk)?;
        Ok(convert::string_out(token))
    };
    cresult(fun())
}

#[unsafe(no_mangle)]
pub extern "C" fn JwtBuilder_created_at_set(builder: *mut JwtBuilder, value: i64) {
    let builder = unsafe { convert::borrow_mut_in(builder) };
    builder.created_at =
        Utc.timestamp_opt(value, 0).single().expect("UTC timestamps are unambiguous");
}

#[unsafe(no_mangle)]
pub extern "C" fn JwtBuilder_created_at_get(builder: *const JwtBuilder) -> i64 {
    let builder = unsafe { convert::borrow_in(builder) };
    builder.created_at.timestamp()
}

#[unsafe(no_mangle)]
pub extern "C" fn JwtBuilder_time_to_live_set(builder: *mut JwtBuilder, value: i64) {
    let builder = unsafe { convert::borrow_mut_in(builder) };
    builder.time_to_live = Duration::seconds(value);
}

#[unsafe(no_mangle)]
pub extern "C" fn JwtBuilder_time_to_live_get(builder: *const JwtBuilder) -> i64 {
    let builder = unsafe { convert::borrow_in(builder) };
    builder.time_to_live.num_seconds()
}
