use super::*;

#[no_mangle]
pub extern "C" fn delete_MorpheusPrivateKind(kind: *mut PrivateKind) {
    if kind.is_null() {
        return;
    }
    let kind = unsafe { Box::from_raw(kind) };
    drop(kind); // NOTE redundant, but clearer than let _kind = ...;
}

// TODO MorpheusPrivateKind_bip32_path_get and MorpheusPrivateKind_network_get

#[no_mangle]
pub extern "C" fn MorpheusPrivateKind_kind_get(kind: *mut PrivateKind) -> *mut raw::c_char {
    let kind = unsafe { convert::borrow_in(kind) };
    let res = format!("{:?}", kind.path());
    convert::string_out(res)
}

#[no_mangle]
pub extern "C" fn MorpheusPrivateKind_len_get(kind: *mut PrivateKind) -> CPtrResult<usize> {
    let kind = unsafe { convert::borrow_in(kind) };
    let fun = || {
        let len = kind.len()?;
        Ok(convert::move_out(len))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusPrivateKind_is_empty_get(kind: *mut PrivateKind) -> CPtrResult<u8> {
    let kind = unsafe { convert::borrow_in(kind) };
    let fun = || {
        let is_empty = kind.is_empty()?;
        Ok(convert::bool_out(is_empty))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusPrivateKind_neuter(kind: *mut PrivateKind) -> *mut PublicKind {
    let kind = unsafe { convert::borrow_in(kind) };
    convert::move_out(kind.neuter())
}

#[no_mangle]
pub extern "C" fn MorpheusPrivateKind_key(
    kind: *mut PrivateKind, idx: i32,
) -> CPtrResult<MorpheusPrivateKey> {
    let kind = unsafe { convert::borrow_mut_in(kind) };
    let mut fun = || {
        let sk = kind.key(idx)?;
        Ok(convert::move_out(sk))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusPrivateKind_did(kind: *mut PrivateKind, idx: i32) -> CPtrResult<Did> {
    let kind = unsafe { convert::borrow_mut_in(kind) };
    let mut fun = || {
        let sk = kind.key(idx)?;
        let did = Did::from(sk.neuter().public_key().key_id());
        Ok(convert::move_out(did))
    };
    cresult(fun())
}
