use super::*;

type NoncedBundleBuilder = Vec<UserOperation>;

#[no_mangle]
pub extern "C" fn delete_NoncedBundleBuilder(builder: *mut NoncedBundleBuilder) {
    delete(builder)
}

#[no_mangle]
pub extern "C" fn NoncedBundleBuilder_new() -> *mut NoncedBundleBuilder {
    convert::move_out(NoncedBundleBuilder::new())
}

#[no_mangle]
pub extern "C" fn NoncedBundleBuilder_add(
    builder: *mut NoncedBundleBuilder, operation: *mut UserOperation,
) {
    let builder = unsafe { convert::borrow_mut_in(builder) };
    let operation = unsafe { convert::borrow_in(operation) };
    builder.push(operation.to_owned());
}

#[no_mangle]
pub extern "C" fn NoncedBundleBuilder_build(
    builder: *mut NoncedBundleBuilder, nonce: Nonce,
) -> *mut NoncedBundle {
    let builder = unsafe { convert::borrow_in(builder) };
    let nonced_bundle = NoncedBundle::new(builder.to_owned(), nonce);
    convert::move_out(nonced_bundle)
}

#[no_mangle]
pub extern "C" fn delete_NoncedBundle(nonced: *mut NoncedBundle) {
    delete(nonced)
}

#[no_mangle]
pub extern "C" fn NoncedBundle_sign(
    nonced_bundle: *const NoncedBundle, sk: *const MPrivateKey,
) -> CPtrResult<SignedBundle> {
    let fun = || {
        let this = unsafe { convert::borrow_in(nonced_bundle) };
        let sk = unsafe { convert::borrow_in(sk) };
        let signed = this.to_owned().sign(sk)?;
        Ok(convert::move_out(signed))
    };
    cresult(fun())
}

// TODO #[no_mangle] pub extern "C" fn NoncedBundle_price(nonced_bundle: *mut NoncedBundle, state: ...) -> CPtrResult<...> {}
// TODO #[no_mangle] pub extern "C" fn NoncedBundle_serialize(nonced_bundle: *mut NoncedBundle) -> CPtrResult<...> {}

#[no_mangle]
pub extern "C" fn delete_SignedBundle(signed: *mut SignedBundle) {
    delete(signed)
}

// TODO #[no_mangle] pub extern "C" fn SignedBundle_price(signed_bundle: *mut SignedBundle, state: ...) -> CPtrResult<...> {}
// TODO #[no_mangle] pub extern "C" fn SignedBundle_verify(signed_bundle: *mut SignedBundle) -> CPtrResult<...> {}
