use super::*;

#[no_mangle]
pub extern "C" fn SecpPrivateKey_sign_hydra_tx(
    sk: *mut SecpPrivateKey, unsigned_tx: *const raw::c_char,
) -> CPtrResult<raw::c_char> {
    let sk = unsafe { convert::borrow_in(sk) };
    let fun = || {
        let tx_str = unsafe { convert::str_in(unsigned_tx)? };
        let mut tx_data: TransactionData = serde_json::from_str(tx_str)?;
        sk.sign_hydra_transaction(&mut tx_data)?;
        let signed_tx_str = serde_json::to_string(&tx_data)?;
        Ok(convert::string_out(signed_tx_str))
    };
    cresult(fun())
}
