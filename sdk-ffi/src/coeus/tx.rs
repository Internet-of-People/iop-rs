use super::*;
use iop_hydra_proto::txtype::Aip29Transaction;

pub struct CoeusTxBuilder {
    network: &'static dyn Network<Suite = Secp256k1>,
}

#[no_mangle]
pub extern "C" fn delete_CoeusTxBuilder(op: *mut CoeusTxBuilder) {
    delete(op)
}

#[no_mangle]
pub extern "C" fn CoeusTxBuilder_new(network: *const raw::c_char) -> CPtrResult<CoeusTxBuilder> {
    let fun = || {
        let network = unsafe { convert::str_in(network) }?;
        let network = Networks::by_name(network)?;
        let builder = CoeusTxBuilder { network };
        Ok(convert::move_out(builder))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn CoeusTxBuilder_build(
    builder: *mut CoeusTxBuilder, bundle: *mut SignedBundle, sender_pubkey: *const SecpPublicKey,
    nonce: Nonce,
) -> CPtrResult<raw::c_char> {
    let fun = || {
        let builder = unsafe { convert::borrow_in(builder) };
        let bundle = unsafe { convert::borrow_in(bundle) };
        let sender_pubkey = unsafe { convert::borrow_in(sender_pubkey) };

        let common_fields = CommonTransactionFields {
            network: builder.network,
            sender_public_key: sender_pubkey.to_owned(),
            nonce,
            optional: Default::default(),
        };

        let tx = coeus::Transaction::new(common_fields, vec![bundle.to_owned()]);
        let tx_json = serde_json::to_string(&tx.to_data())?;
        Ok(convert::string_out(tx_json))
    };
    cresult(fun())
}
