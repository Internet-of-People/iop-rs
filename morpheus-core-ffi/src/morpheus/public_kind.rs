use super::*;

#[no_mangle]
pub extern "C" fn delete_MorpheusPublicKind(kind: *mut PublicKind) {
    if kind.is_null() {
        return;
    }
    let kind = unsafe { Box::from_raw(kind) };
    drop(kind); // NOTE redundant, but clearer than let _kind = ...;
}

#[no_mangle]
pub extern "C" fn MorpheusPublicKind_len_get(kind: *mut PublicKind) -> CPtrResult<usize> {
    let kind = unsafe { convert::borrow_in(kind) };
    let fun = || {
        let len = kind.len()?;
        Ok(convert::move_out(len))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusPublicKind_is_empty_get(kind: *mut PublicKind) -> CPtrResult<u8> {
    let kind = unsafe { convert::borrow_in(kind) };
    let fun = || {
        let is_empty = kind.is_empty()?;
        Ok(convert::bool_out(is_empty))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusPublicKind_key(
    kind: *mut PublicKind, idx: i32,
) -> CPtrResult<MPublicKey> {
    let kind = unsafe { convert::borrow_mut_in(kind) };
    let fun = || {
        let sk = kind.key(idx)?;
        Ok(convert::move_out(sk))
    };
    cresult(fun())
}
