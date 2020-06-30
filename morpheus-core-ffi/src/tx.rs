use super::*;

use iop_keyvault::Networks;
use iop_morpheus_core::hydra::txtype::{core, morpheus, Aip29Transaction, CommonTransactionFields};

#[no_mangle]
pub extern "C" fn TxBuilder_hydraTransferTx(
    network: *const raw::c_char, sender_pubkey: *const raw::c_char, recipient: *const raw::c_char,
    amount: u64, nonce: u64,
) -> CPtrResult<raw::c_char> {
    let fun = || {
        let network = Networks::by_name(convert::str_in(network)?)?;
        let recipient_id = convert::str_in(recipient)?.to_owned();
        let sender_public_key = convert::str_in(sender_pubkey)?.to_owned();
        let common_fields = CommonTransactionFields {
            network,
            sender_public_key,
            amount,
            nonce,
            ..Default::default()
        };
        let transfer_tx = core::TransferTransaction::new(common_fields, recipient_id);
        let tx_str = serde_json::to_string(&transfer_tx.to_data())?;
        Ok(convert::string_out(tx_str))
    };
    cresult(fun())
}
