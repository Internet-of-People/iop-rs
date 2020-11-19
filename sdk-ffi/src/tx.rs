use super::*;

use iop_hydra_proto::txtype::{
    hyd_core, morpheus, Aip29Transaction, CommonTransactionFields, OptionalTransactionFields,
};
use iop_keyvault::Networks;

#[no_mangle]
pub extern "C" fn HydraTxBuilder_transfer(
    network: *const raw::c_char, sender_public_key: *const SecpPublicKey,
    recipient_id: *const raw::c_char, amount: u64, nonce: u64,
) -> CPtrResult<raw::c_char> {
    let fun = || {
        let network = unsafe { convert::str_in(network)? };
        let recipient_id = unsafe { convert::str_in(recipient_id)? };
        let sender_public_key = unsafe { convert::borrow_in(sender_public_key) };
        let network = Networks::by_name(network)?;
        let common_fields = CommonTransactionFields {
            network,
            sender_public_key: sender_public_key.to_owned(),
            nonce,
            optional: OptionalTransactionFields { amount, ..Default::default() },
        };
        let recipient_id = SecpKeyId::from_p2pkh_addr(recipient_id, network)?;
        let transfer_tx = hyd_core::Transaction::transfer(common_fields, &recipient_id);
        let tx_str = serde_json::to_string(&transfer_tx.to_data())?;
        Ok(convert::string_out(tx_str))
    };
    cresult(fun())
}

fn create_vote_tx(
    network: *const raw::c_char, sender_public_key: *const SecpPublicKey,
    delegate: *const SecpPublicKey, nonce: u64,
    build_tx: fn(CommonTransactionFields, &SecpPublicKey) -> hyd_core::Transaction,
) -> CPtrResult<raw::c_char> {
    let fun = || {
        let network = unsafe { convert::str_in(network)? };
        let delegate = unsafe { convert::borrow_in(delegate) };
        let sender_public_key = unsafe { convert::borrow_in(sender_public_key) };
        let common_fields = CommonTransactionFields {
            network: Networks::by_name(network)?,
            sender_public_key: sender_public_key.to_owned(),
            nonce,
            optional: Default::default(),
        };
        let vote_tx = build_tx(common_fields, delegate);
        let tx_str = serde_json::to_string(&vote_tx.to_data())?;
        Ok(convert::string_out(tx_str))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn HydraTxBuilder_vote(
    network: *const raw::c_char, sender_public_key: *const SecpPublicKey,
    delegate: *const SecpPublicKey, nonce: u64,
) -> CPtrResult<raw::c_char> {
    create_vote_tx(network, sender_public_key, delegate, nonce, hyd_core::Transaction::vote)
}

#[no_mangle]
pub extern "C" fn HydraTxBuilder_unvote(
    network: *const raw::c_char, sender_public_key: *const SecpPublicKey,
    delegate: *const SecpPublicKey, nonce: u64,
) -> CPtrResult<raw::c_char> {
    create_vote_tx(network, sender_public_key, delegate, nonce, hyd_core::Transaction::unvote)
}

#[no_mangle]
pub extern "C" fn HydraTxBuilder_register_delegate(
    network: *const raw::c_char, sender_public_key: *const SecpPublicKey,
    delegate_name: *const raw::c_char, nonce: u64,
) -> CPtrResult<raw::c_char> {
    let fun = || {
        let network = unsafe { convert::str_in(network)? };
        let sender_public_key = unsafe { convert::borrow_in(sender_public_key) };
        let delegate_name = unsafe { convert::str_in(delegate_name)? };
        let network = Networks::by_name(network)?;
        let common_fields = CommonTransactionFields {
            network,
            sender_public_key: sender_public_key.to_owned(),
            nonce,
            optional: Default::default(),
        };
        let reg_tx = hyd_core::Transaction::register_delegate(common_fields, delegate_name);
        let tx_str = serde_json::to_string(&reg_tx.to_data())?;
        Ok(convert::string_out(tx_str))
    };
    cresult(fun())
}

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
