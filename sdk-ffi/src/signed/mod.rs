mod bytes;
mod json;

use super::*;

fn public_key<T: Signable>(signed: *mut Signed<T>) -> *mut MPublicKey {
    let signed = unsafe { convert::borrow_in(signed) };
    let pk = signed.public_key().to_owned();
    convert::move_out(pk)
}

fn signature<T: Signable>(signed: *mut Signed<T>) -> *mut MSignature {
    let signed = unsafe { convert::borrow_in(signed) };
    let sig = signed.signature().to_owned();
    convert::move_out(sig)
}

fn validate_with_did_doc<T: Signable>(
    signed: *const Signed<T>, did_doc_str: *const raw::c_char, from_height_inc: *const BlockHeight,
    until_height_exc: *const BlockHeight,
) -> CPtrResult<ValidationResult> {
    let signed = unsafe { convert::borrow_in(signed) };
    let from_height_inc = unsafe { convert::borrow_in_opt(from_height_inc) };
    let until_height_exc = unsafe { convert::borrow_in_opt(until_height_exc) };
    let fun = || {
        let did_doc_str = unsafe { convert::str_in(did_doc_str)? };
        let did_doc = serde_json::from_str(did_doc_str)?;
        let validation_result = signed.validate_with_did_doc(
            &did_doc,
            from_height_inc.cloned(),
            until_height_exc.cloned(),
        )?;
        Ok(convert::move_out(validation_result))
    };
    cresult(fun())
}
