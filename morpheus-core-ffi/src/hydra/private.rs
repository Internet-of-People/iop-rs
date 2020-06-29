use super::*;

use iop_morpheus_core::hydra::transaction::TransactionData;

#[no_mangle]
pub extern "C" fn HydraPlugin_private(
    hydra: *mut CHydraPlugin, unlock_pwd: *const raw::c_char,
) -> CPtrResult<Private> {
    let hydra = unsafe { convert::borrow_in(hydra) };
    let fun = || {
        let unlock_password = convert::str_in(unlock_pwd)?;
        let private = hydra.plugin.private(unlock_password)?;
        Ok(convert::move_out(private))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn delete_HydraPrivate(private: *mut Private) {
    if private.is_null() {
        return;
    }
    let private = unsafe { Box::from_raw(private) };
    drop(private); // NOTE redundant, but clearer than let _plugin = ...;
}

// TODO move these infrastructural features into morpheus-core, especially for potential reuse in Wasm
// TODO hyd_addr should be typed and be valid after parsing
// TODO tx should be typed and be a result of some HydraTxBuilder
#[no_mangle]
pub extern "C" fn HydraPrivate_sign_hydra_tx(
    private: *mut Private, hyd_addr: *const raw::c_char, unsigned_tx: *const raw::c_char,
) -> CPtrResult<raw::c_char> {
    let private = unsafe { convert::borrow_mut_in(private) };
    let fun = || {
        let hyd_addr = convert::str_in(hyd_addr)?;
        let tx_str = convert::str_in(unsigned_tx)?;
        let mut tx_data: TransactionData = serde_json::from_str(tx_str)?;
        private.sign_hydra_transaction(hyd_addr, &mut tx_data)?;
        let signed_tx_str = serde_json::to_string(&tx_data)?;
        Ok(convert::string_out(signed_tx_str))
    };
    cresult(fun())
}
