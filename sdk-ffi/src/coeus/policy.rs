use super::*;

#[no_mangle]
pub extern "C" fn delete_SubtreePolicies(op: *mut SubtreePolicies) {
    delete(op)
}

#[no_mangle]
pub extern "C" fn SubtreePolicies_new() -> *mut SubtreePolicies {
    convert::move_out(SubtreePolicies::new())
}

#[no_mangle]
pub extern "C" fn SubtreePolicies_with_schema(
    policies: *mut SubtreePolicies, schema: *const raw::c_char,
) -> CPtrResult<SubtreePolicies> {
    let fun = || {
        let this = unsafe { convert::borrow_in(policies) };
        let schema = unsafe { convert::str_in(schema)? }.parse()?;
        let this = this.clone().with_schema(schema);
        Ok(convert::move_out(this))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn SubtreePolicies_with_expiration(
    policies: *mut SubtreePolicies, max_expiry: BlockCount,
) -> CPtrResult<SubtreePolicies> {
    let fun = || {
        let this = unsafe { convert::borrow_in(policies) };
        let this = this.clone().with_expiration(max_expiry);
        Ok(convert::move_out(this))
    };
    cresult(fun())
}
