use super::*;

#[no_mangle]
pub extern "C" fn delete_MorpheusPrivate(private: *mut Private) {
    delete(private)
}

#[no_mangle]
pub extern "C" fn MorpheusPrivate_personas_get(private: *mut Private) -> CPtrResult<PrivateKind> {
    let private = unsafe { convert::borrow_in(private) };
    let fun = || {
        let kind = private.personas()?;
        Ok(convert::move_out(kind))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusPrivate_public_get(private: *mut Private) -> *mut Public {
    let private = unsafe { convert::borrow_in(private) };
    convert::move_out(private.public())
}

#[no_mangle]
pub extern "C" fn MorpheusPrivate_key_by_pk(
    private: *mut Private, pk: *mut MPublicKey,
) -> CPtrResult<MorpheusPrivateKey> {
    let private = unsafe { convert::borrow_in(private) };
    let pk = unsafe { convert::borrow_in(pk) };
    let fun = || {
        let sk = private.key_by_pk(pk)?;
        Ok(convert::move_out(sk))
    };
    cresult(fun())
}
