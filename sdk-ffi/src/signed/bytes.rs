use super::*;

#[no_mangle]
pub extern "C" fn delete_SignedBytes(signed: *mut Signed<Box<[u8]>>) {
    delete(signed)
}

#[no_mangle]
pub extern "C" fn SignedBytes_new(
    pk: *const MPublicKey, content: *const CSlice<u8>, sig: *const MSignature,
) -> *mut Signed<Box<[u8]>> {
    let pk = unsafe { convert::borrow_in(pk) };
    let content = unsafe { convert::borrow_in(content) };
    let sig = unsafe { convert::borrow_in(sig) };
    let signed_bytes =
        Signed::new(pk.clone(), content.as_slice().to_owned().into_boxed_slice(), sig.clone());
    convert::move_out(signed_bytes)
}

#[no_mangle]
pub extern "C" fn SignedBytes_public_key_get(signed: *mut Signed<Box<[u8]>>) -> *mut MPublicKey {
    public_key(signed)
}

#[no_mangle]
pub extern "C" fn SignedBytes_content_get(signed: *mut Signed<Box<[u8]>>) -> *mut CSlice<u8> {
    let signed = unsafe { convert::borrow_in(signed) };
    let slice = signed.content().as_ref().to_owned();
    convert::move_out(CSlice::from(slice))
}

#[no_mangle]
pub extern "C" fn SignedBytes_signature_get(signed: *mut Signed<Box<[u8]>>) -> *mut MSignature {
    signature(signed)
}

#[no_mangle]
pub extern "C" fn SignedBytes_validate(signed: *mut Signed<Box<[u8]>>) -> bool {
    let signed = unsafe { convert::borrow_in(signed) };
    signed.validate()
}

#[no_mangle]
pub extern "C" fn SignedBytes_validate_with_keyid(
    signed: *mut Signed<Box<[u8]>>, signer_id: *mut MKeyId,
) -> bool {
    let signed = unsafe { convert::borrow_in(signed) };
    let signer_id = unsafe { convert::borrow_in(signer_id) };
    signed.validate_with_keyid(Some(signer_id))
}

#[no_mangle]
pub extern "C" fn SignedBytes_validate_with_did_doc(
    signed: *mut Signed<Box<[u8]>>, did_doc_str: *const raw::c_char,
    from_height_inc: *const BlockHeight, until_height_exc: *const BlockHeight,
) -> CPtrResult<ValidationResult> {
    validate_with_did_doc(signed, did_doc_str, from_height_inc, until_height_exc)
}
