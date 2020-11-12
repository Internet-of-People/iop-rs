use super::*;

type CoeusTxBuilder = &'static dyn Network<Suite = Secp256k1>;

#[no_mangle]
pub extern "C" fn delete_CoeusTxBuilder(op: *mut CoeusTxBuilder) {
    delete(op)
}

// TODO
// HydraSigner
