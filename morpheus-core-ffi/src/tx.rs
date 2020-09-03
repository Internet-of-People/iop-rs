use super::*;

use iop_keyvault::Networks;
use iop_morpheus_core::hydra::txtype::{
    hyd_core, morpheus, Aip29Transaction, CommonTransactionFields,
};

#[no_mangle]
pub extern "C" fn HydraTxBuilder_transfer(
    network: *const raw::c_char, sender_public_key: *const raw::c_char,
    recipient_id: *const raw::c_char, amount: u64, nonce: u64,
) -> CPtrResult<raw::c_char> {
    let fun = || {
        let network = unsafe { convert::str_in(network)? };
        let recipient_id = unsafe { convert::str_in(recipient_id)? };
        let sender_public_key = unsafe { convert::str_in(sender_public_key)? };
        let common_fields = CommonTransactionFields {
            network: Networks::by_name(network)?,
            sender_public_key: sender_public_key.to_owned(),
            amount,
            nonce,
            ..Default::default()
        };
        let transfer_tx = hyd_core::Transaction::transfer(common_fields, recipient_id.to_owned());
        let tx_str = serde_json::to_string(&transfer_tx.to_data())?;
        Ok(convert::string_out(tx_str))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn HydraTxBuilder_vote(
    network: *const raw::c_char, sender_public_key: *const raw::c_char, vote: *const raw::c_char,
    nonce: u64,
) -> CPtrResult<raw::c_char> {
    let fun = || {
        let network = unsafe { convert::str_in(network)? };
        let vote = unsafe { convert::str_in(vote)? };
        let sender_public_key = unsafe { convert::str_in(sender_public_key)? };
        let common_fields = CommonTransactionFields {
            network: Networks::by_name(network)?,
            sender_public_key: sender_public_key.to_owned(),
            nonce,
            ..Default::default()
        };
        let vote_tx = hyd_core::Transaction::vote(common_fields, vote);
        let tx_str = serde_json::to_string(&vote_tx.to_data())?;
        Ok(convert::string_out(tx_str))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn MorpheusTxBuilder_new(
    network: *const raw::c_char, sender_public_key: *const raw::c_char,
    attempts: *const raw::c_char, nonce: u64,
) -> CPtrResult<raw::c_char> {
    let fun = || {
        let network = unsafe { convert::str_in(network)? };
        let attempts = unsafe { convert::str_in(attempts)? };
        let sender_public_key = unsafe { convert::str_in(sender_public_key)? };
        let op_attempts: Vec<morpheus::OperationAttempt> = serde_json::from_str(attempts)?;
        let common_fields = CommonTransactionFields {
            network: Networks::by_name(network)?,
            sender_public_key: sender_public_key.to_owned(),
            nonce,
            ..Default::default()
        };
        let transfer_tx = morpheus::Transaction::new(common_fields, op_attempts);
        let tx_str = serde_json::to_string(&transfer_tx.to_data())?;
        Ok(convert::string_out(tx_str))
    };
    cresult(fun())
}
