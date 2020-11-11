use super::*;

#[no_mangle]
pub extern "C" fn delete_UserOperation(op: *mut UserOperation) {
    delete(op)
}

#[no_mangle]
pub extern "C" fn UserOperation_register(
    domain: *const raw::c_char, owner: *const raw::c_char, subtree_policies: *mut SubtreePolicies,
    data: *const raw::c_char, expires_at_height: BlockHeight,
) -> CPtrResult<UserOperation> {
    let fun = || {
        let domain = unsafe { convert::str_in(domain)? }.parse()?;
        let owner = unsafe { convert::str_in(owner)? }.parse()?;
        let subtree_policies = unsafe { convert::borrow_in(subtree_policies) };
        let data = unsafe { convert::str_in(data)? }.parse()?;
        let op = UserOperation::register(
            domain,
            owner,
            subtree_policies.clone(),
            RegistrationPolicy::default(),
            data,
            expires_at_height,
        );
        Ok(convert::move_out(op))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn UserOperation_update(
    domain: *const raw::c_char, data: *const raw::c_char,
) -> CPtrResult<UserOperation> {
    let fun = || {
        let domain = unsafe { convert::str_in(domain)? }.parse()?;
        let data = unsafe { convert::str_in(data)? }.parse()?;
        let op = UserOperation::update(domain, data);
        Ok(convert::move_out(op))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn UserOperation_renew(
    domain: *const raw::c_char, expires_at_height: BlockHeight,
) -> CPtrResult<UserOperation> {
    let fun = || {
        let domain = unsafe { convert::str_in(domain)? }.parse()?;
        let op = UserOperation::renew(domain, expires_at_height);
        Ok(convert::move_out(op))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn UserOperation_transfer(
    domain: *const raw::c_char, to_owner: *const raw::c_char,
) -> CPtrResult<UserOperation> {
    let fun = || {
        let domain = unsafe { convert::str_in(domain)? }.parse()?;
        let to_owner = unsafe { convert::str_in(to_owner)? }.parse()?;
        let op = UserOperation::transfer(domain, to_owner);
        Ok(convert::move_out(op))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn UserOperation_delete(domain: *const raw::c_char) -> CPtrResult<UserOperation> {
    let fun = || {
        let domain = unsafe { convert::str_in(domain)? }.parse()?;
        let op = UserOperation::delete(domain);
        Ok(convert::move_out(op))
    };
    cresult(fun())
}
