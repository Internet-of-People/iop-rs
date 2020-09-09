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

#[no_mangle]
pub extern "C" fn MorpheusPrivate_sign_did_operations(
    private: *mut Private, id: *mut MKeyId, message: *mut CSlice<u8>,
) -> CPtrResult<Signed<Box<[u8]>>> {
    let private = unsafe { convert::borrow_in(private) };
    let id = unsafe { convert::borrow_in(id) };
    let message = unsafe { convert::borrow_in(message) };
    let fun = || {
        let signer = create_signer(private, id)?;
        let (public_key, signature) = signer.sign(message.as_slice())?;
        let signed_bytes =
            Signed::new(public_key, message.as_slice().to_owned().into_boxed_slice(), signature);
        Ok(convert::move_out(signed_bytes))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusPrivate_sign_witness_request(
    private: *mut Private, id: *mut MKeyId, request: *mut raw::c_char,
) -> CPtrResult<Signed<serde_json::Value>> {
    let private = unsafe { convert::borrow_in(private) };
    let id = unsafe { convert::borrow_in(id) };
    let fun = || {
        let request = unsafe { convert::str_in(request)? };
        let signer = create_signer(private, id)?;
        let request: WitnessRequest = serde_json::from_str(request)?;
        let signed_request = signer.sign_witness_request(request)?;
        let signed_json = into_signed_json(signed_request)?;
        Ok(convert::move_out(signed_json))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusPrivate_sign_witness_statement(
    private: *mut Private, id: *mut MKeyId, statement: *mut raw::c_char,
) -> CPtrResult<Signed<serde_json::Value>> {
    let private = unsafe { convert::borrow_in(private) };
    let id = unsafe { convert::borrow_in(id) };
    let fun = || {
        let statement = unsafe { convert::str_in(statement)? };
        let signer = create_signer(private, id)?;
        let statement: WitnessStatement = serde_json::from_str(statement)?;
        let signed_statement = signer.sign_witness_statement(statement)?;
        let signed_json = into_signed_json(signed_statement)?;
        Ok(convert::move_out(signed_json))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusPrivate_sign_claim_presentation(
    private: *mut Private, id: *mut MKeyId, presentation: *mut raw::c_char,
) -> CPtrResult<Signed<serde_json::Value>> {
    let private = unsafe { convert::borrow_in(private) };
    let id = unsafe { convert::borrow_in(id) };
    let fun = || {
        let presentation = unsafe { convert::str_in(presentation)? };
        let signer = create_signer(private, id)?;
        let presentation: ClaimPresentation = serde_json::from_str(presentation)?;
        let signed_presentation = signer.sign_claim_presentation(presentation)?;
        let signed_json = into_signed_json(signed_presentation)?;
        Ok(convert::move_out(signed_json))
    };
    cresult(fun())
}

fn create_signer(private: &Private, id: &MKeyId) -> Result<PrivateKeySigner> {
    let sk: MPrivateKey = key_by_id(private, id)?.private_key();
    Ok(PrivateKeySigner::new(sk))
}

fn key_by_id(private: &Private, id: &MKeyId) -> Result<MorpheusPrivateKey> {
    let pk = private.public().key_by_id(id)?;
    let morpheus_sk = private.key_by_pk(&pk)?;
    Ok(morpheus_sk)
}

fn into_signed_json<T: Signable>(signed: Signed<T>) -> Result<Signed<serde_json::Value>> {
    let (public_key, content, signature, nonce) = signed.into_parts();
    let content = serde_json::to_value(content)?;
    let signed_json = Signed::from_parts(public_key, content, signature, nonce);
    Ok(signed_json)
}
