use super::*;

#[no_mangle]
pub extern "C" fn MorpheusPlugin_private(
    morpheus: *mut CMorpheusPlugin, unlock_pwd: *const raw::c_char,
) -> CPtrResult<Private> {
    let morpheus = unsafe { convert::borrow_in(morpheus) };
    let fun = || {
        let unlock_password = unsafe { convert::str_in(unlock_pwd)? };
        let private = morpheus.plugin.private(unlock_password)?;
        Ok(convert::move_out(private))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn delete_MorpheusPrivate(private: *mut Private) {
    if private.is_null() {
        return;
    }
    let private = unsafe { Box::from_raw(private) };
    drop(private); // NOTE redundant, but clearer than let _plugin = ...;
}

#[no_mangle]
pub extern "C" fn MorpheusPrivate_personas(private: *mut Private) -> CPtrResult<PrivateKind> {
    let private = unsafe { convert::borrow_in(private) };
    let fun = || {
        let kind = private.personas()?;
        Ok(convert::move_out(kind))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusPrivate_neuter(private: *mut Private) -> *mut Public {
    let private = unsafe { convert::borrow_in(private) };
    convert::move_out(private.neuter())
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
