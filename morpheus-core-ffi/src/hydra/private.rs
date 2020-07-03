use super::*;

use iop_morpheus_core::hydra::transaction::TransactionData;

#[no_mangle]
pub extern "C" fn HydraPlugin_private(
    hydra: *mut CHydraPlugin, unlock_pwd: *const raw::c_char,
) -> CPtrResult<Private> {
    let hydra = unsafe { convert::borrow_in(hydra) };
    let fun = || {
        let unlock_password = unsafe { convert::str_in(unlock_pwd)? };
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

#[no_mangle]
pub extern "C" fn HydraPrivate_neuter(private: *mut Private) -> CPtrResult<Public> {
    let fun = || {
        let private = unsafe { convert::borrow_in(private) };
        let public = private.neuter();
        Ok(convert::move_out(public))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn HydraPrivate_xpub_get(private: *mut Private) -> CPtrResult<raw::c_char> {
    let private = unsafe { convert::borrow_in(private) };
    let fun = || {
        let xpub = private.xpub()?;
        Ok(convert::string_out(xpub))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn HydraPrivate_receive_keys_get(private: *mut Private) -> CPtrResult<u32> {
    let private = unsafe { convert::borrow_in(private) };
    let fun = || {
        let receive_keys = private.receive_keys()?;
        Ok(convert::move_out(receive_keys))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn HydraPrivate_change_keys_get(private: *mut Private) -> CPtrResult<u32> {
    let private = unsafe { convert::borrow_in(private) };
    let fun = || {
        let change_keys = private.change_keys()?;
        Ok(convert::move_out(change_keys))
    };
    cresult(fun())
}

// TODO consider using strong typing for tx
#[no_mangle]
pub extern "C" fn HydraPrivate_sign_hydra_tx(
    private: *mut Private, hyd_addr: *const raw::c_char, unsigned_tx: *const raw::c_char,
) -> CPtrResult<raw::c_char> {
    let private = unsafe { convert::borrow_mut_in(private) };
    let fun = || {
        let hyd_addr = unsafe { convert::str_in(hyd_addr)? };
        let tx_str = unsafe { convert::str_in(unsigned_tx)? };
        let mut tx_data: TransactionData = serde_json::from_str(tx_str)?;
        private.sign_hydra_transaction(hyd_addr, &mut tx_data)?;
        let signed_tx_str = serde_json::to_string(&tx_data)?;
        Ok(convert::string_out(signed_tx_str))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn HydraPrivate_key(
    private: *mut Private, idx: i32,
) -> CPtrResult<Bip44Key<Secp256k1>> {
    let fun = || {
        let private = unsafe { convert::borrow_mut_in(private) };
        let key = private.key(idx)?;
        Ok(convert::move_out(key))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn HydraPrivate_key_by_pk(
    private: *mut Private, pub_key: *mut SecpPublicKey,
) -> CPtrResult<Bip44Key<Secp256k1>> {
    let private = unsafe { convert::borrow_in(private) };
    let pub_key = unsafe { convert::borrow_in(pub_key) };
    let fun = || {
        let key = private.key_by_pk(&pub_key)?;
        Ok(convert::move_out(key))
    };
    cresult(fun())
}
