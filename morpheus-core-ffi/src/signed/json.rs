use super::*;

#[no_mangle]
pub extern "C" fn delete_SignedJson(signed: *mut Signed<serde_json::Value>) {
    delete(signed)
}

#[no_mangle]
pub extern "C" fn SignedJson_public_key_get(
    signed: *mut Signed<serde_json::Value>,
) -> *mut MPublicKey {
    public_key(signed)
}

#[no_mangle]
pub extern "C" fn SignedJson_content_get(
    signed: *mut Signed<serde_json::Value>,
) -> *mut raw::c_char {
    let signed = unsafe { convert::borrow_in(signed) };
    // The serialization of Value will not fail, we do not have non-string keys in maps.
    let string = serde_json::to_string_pretty(signed.content()).unwrap();
    convert::string_out(string)
}

#[no_mangle]
pub extern "C" fn SignedJson_signature_get(
    signed: *mut Signed<serde_json::Value>,
) -> *mut MSignature {
    signature(signed)
}

#[no_mangle]
pub extern "C" fn SignedJson_validate(signed: *mut Signed<serde_json::Value>) -> bool {
    let signed = unsafe { convert::borrow_in(signed) };
    signed.validate()
}

#[no_mangle]
pub extern "C" fn SignedJson_validate_with_keyid(
    signed: *mut Signed<serde_json::Value>, signer_id: *mut MKeyId,
) -> bool {
    let signed = unsafe { convert::borrow_in(signed) };
    let signer_id = unsafe { convert::borrow_in(signer_id) };
    signed.validate_with_keyid(Some(signer_id))
}

#[no_mangle]
pub extern "C" fn SignedJson_validate_with_did_doc(
    signed: *mut Signed<serde_json::Value>, did_doc_str: *const raw::c_char,
    from_height_inc: *const BlockHeight, until_height_exc: *const BlockHeight,
) -> CPtrResult<ValidationResult> {
    validate_with_did_doc(signed, did_doc_str, from_height_inc, until_height_exc)
}
