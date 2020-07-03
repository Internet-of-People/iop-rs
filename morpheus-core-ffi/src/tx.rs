use super::*;

use iop_keyvault::Networks;
use iop_morpheus_core::hydra::txtype::{hyd_core, Aip29Transaction, CommonTransactionFields};

#[no_mangle]
pub extern "C" fn TxBuilder_hydraTransferTx(
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
        let transfer_tx =
            hyd_core::TransferTransaction::new(common_fields, recipient_id.to_owned());
        let tx_str = serde_json::to_string(&transfer_tx.to_data())?;
        Ok(convert::string_out(tx_str))
    };
    cresult(fun())
}
