use super::*;

#[no_mangle]
pub extern "C" fn delete_MorpheusPublicKind(kind: *mut MorpheusPublicKind) {
    delete(kind)
}

#[no_mangle]
pub extern "C" fn MorpheusPublicKind_len_get(kind: *mut MorpheusPublicKind) -> CPtrResult<usize> {
    let kind = unsafe { convert::borrow_in(kind) };
    let fun = || {
        let len = kind.len()?;
        Ok(convert::move_out(len as usize))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusPublicKind_is_empty_get(kind: *mut MorpheusPublicKind) -> CPtrResult<u8> {
    let kind = unsafe { convert::borrow_in(kind) };
    let fun = || {
        let is_empty = kind.is_empty()?;
        Ok(convert::bool_out(is_empty))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusPublicKind_key(
    kind: *mut MorpheusPublicKind, idx: i32,
) -> CPtrResult<MPublicKey> {
    let kind = unsafe { convert::borrow_mut_in(kind) };
    let fun = || {
        let sk = kind.key(idx)?;
        Ok(convert::move_out(sk))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusPublicKind_did(
    kind: *const MorpheusPublicKind, idx: i32,
) -> CPtrResult<Did> {
    let kind = unsafe { convert::borrow_in(kind) };
    let fun = || {
        let pk = kind.key(idx)?;
        let did = Did::from(pk.key_id());
        Ok(convert::move_out(did))
    };
    cresult(fun())
}
