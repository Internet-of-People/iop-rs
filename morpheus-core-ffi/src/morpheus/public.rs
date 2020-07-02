use super::*;

#[no_mangle]
pub extern "C" fn MorpheusPlugin_public(morpheus: *mut CMorpheusPlugin) -> CPtrResult<Public> {
    let morpheus = unsafe { convert::borrow_in(morpheus) };
    let fun = || {
        let public = morpheus.plugin.public()?;
        Ok(convert::move_out(public))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn delete_MorpheusPublic(public: *mut Public) {
    if public.is_null() {
        return;
    }
    let public = unsafe { Box::from_raw(public) };
    drop(public); // NOTE redundant, but clearer than let _public = ...;
}

#[no_mangle]
pub extern "C" fn MorpheusPublic_personas(public: *mut Public) -> CPtrResult<PublicKind> {
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
