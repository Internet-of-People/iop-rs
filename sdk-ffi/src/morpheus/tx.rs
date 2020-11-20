use super::*;

use iop_hydra_proto::txtype::morpheus;

#[no_mangle]
pub extern "C" fn MorpheusTxBuilder_new(
    network: *const raw::c_char, sender_public_key: *const SecpPublicKey,
    attempts: *const raw::c_char, nonce: u64,
) -> CPtrResult<raw::c_char> {
    let fun = || {
        let network = unsafe { convert::str_in(network)? };
        let attempts = unsafe { convert::str_in(attempts)? };
        let sender_public_key = unsafe { convert::borrow_in(sender_public_key) };
        let op_attempts: Vec<morpheus::OperationAttempt> = serde_json::from_str(attempts)?;
        let common_fields = CommonTransactionFields {
            network: Networks::by_name(network)?,
            sender_public_key: sender_public_key.to_owned(),
            nonce,
            optional: Default::default(),
        };
        let morpheus_tx = morpheus::Transaction::new(common_fields, op_attempts);
        let tx_str = serde_json::to_string(&morpheus_tx.to_data())?;
        Ok(convert::string_out(tx_str))
    };
    cresult(fun())
}
