use super::*;

#[no_mangle]
pub extern "C" fn delete_MorpheusPublic(public: *mut Public) {
    delete(public)
}

#[no_mangle]
pub extern "C" fn MorpheusPublic_personas_get(public: *mut Public) -> CPtrResult<PublicKind> {
    let public = unsafe { convert::borrow_in(public) };
    let fun = || {
        let personas = public.personas()?;
        Ok(convert::move_out(personas))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusPublic_key_by_id(
    public: *mut Public, id: *mut MKeyId,
) -> CPtrResult<MPublicKey> {
    let public = unsafe { convert::borrow_in(public) };
    let id = unsafe { convert::borrow_in(id) };
    let fun = || {
        let pk = public.key_by_id(&id)?;
        Ok(convert::move_out(pk))
    };
    cresult(fun())
}
