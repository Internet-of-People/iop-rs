use super::*;

#[no_mangle]
pub extern "C" fn delete_MorpheusPublic(public: *mut MorpheusPublic) {
    delete(public)
}

#[no_mangle]
pub extern "C" fn MorpheusPublic_kind(
    public: *mut MorpheusPublic, did_kind: *const raw::c_char,
) -> CPtrResult<MorpheusPublicKind> {
    let public = unsafe { convert::borrow_in(public) };
    let fun = || {
        let did_kind = unsafe { convert::str_in(did_kind)? };
        let did_kind: DidKind = did_kind.parse()?;
        let kind = public.kind(did_kind)?;
        Ok(convert::move_out(kind))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusPublic_personas_get(
    public: *mut MorpheusPublic,
) -> CPtrResult<MorpheusPublicKind> {
    let public = unsafe { convert::borrow_in(public) };
    let fun = || {
        let kind = public.personas()?;
        Ok(convert::move_out(kind))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusPublic_devices_get(
    public: *mut MorpheusPublic,
) -> CPtrResult<MorpheusPublicKind> {
    let public = unsafe { convert::borrow_in(public) };
    let fun = || {
        let kind = public.devices()?;
        Ok(convert::move_out(kind))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusPublic_groups_get(
    public: *mut MorpheusPublic,
) -> CPtrResult<MorpheusPublicKind> {
    let public = unsafe { convert::borrow_in(public) };
    let fun = || {
        let kind = public.groups()?;
        Ok(convert::move_out(kind))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusPublic_resources_get(
    public: *mut MorpheusPublic,
) -> CPtrResult<MorpheusPublicKind> {
    let public = unsafe { convert::borrow_in(public) };
    let fun = || {
        let kind = public.resources()?;
        Ok(convert::move_out(kind))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusPublic_key_by_id(
    public: *mut MorpheusPublic, id: *mut MKeyId,
) -> CPtrResult<MPublicKey> {
    let public = unsafe { convert::borrow_in(public) };
    let id = unsafe { convert::borrow_in(id) };
    let fun = || {
        let pk = public.key_by_id(&id)?;
        Ok(convert::move_out(pk))
    };
    cresult(fun())
}
