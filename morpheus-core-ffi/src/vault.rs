use super::*;

pub struct CVault {
    inner: Vault,
}

impl CVault {
    pub fn is_dirty(&self) -> Fallible<bool> {
        let flag_state = self.inner.to_modifiable();
        let dirty_flag_value = flag_state.try_borrow()?;
        Ok(*dirty_flag_value)
    }

    pub fn set_dirty(&self, value: bool) -> Fallible<()> {
        let mut vault_state = self.inner.to_modifiable();
        let mut dirty_flag = vault_state.try_borrow_mut()?;
        *dirty_flag = value;
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn Vault_create(
    lang: *const raw::c_char, seed: *const raw::c_char, word25: *const raw::c_char,
    unlock_pwd: *const raw::c_char,
) -> CPtrResult<CVault> {
    let fun = || {
        let lang = convert::str_in(lang)?;
        let seed = convert::str_in(seed)?;
        let bip39_password = convert::str_in(word25)?;
        let unlock_password = convert::str_in(unlock_pwd)?;
        let inner = Vault::create(Some(lang), seed, bip39_password, unlock_password)?;
        let vault = CVault { inner };
        Ok(convert::move_out(vault))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn Vault_load(json: *const raw::c_char) -> CPtrResult<CVault> {
    let fun = || {
        let json = convert::str_in(json)?;
        let inner: Vault = serde_json::from_str(json)?;
        let vault = CVault { inner };
        Ok(convert::move_out(vault))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn delete_Vault(vault: *mut CVault) {
    if vault.is_null() {
        return;
    }
    let vault = unsafe { Box::from_raw(vault) };
    drop(vault); // NOTE redundant, but clearer than let _vault = ...;
}

#[no_mangle]
pub extern "C" fn Vault_save(vault: *mut CVault) -> CPtrResult<raw::c_char> {
    let vault = unsafe { convert::borrow_in(vault) };
    let fun = || {
        let vault_json = serde_json::to_string(&vault.inner)?;
        vault.set_dirty(false)?;
        Ok(convert::string_out(vault_json))
    };
    cresult(fun())
}

#[no_mangle]
pub extern "C" fn Vault_dirty_get(vault: *mut CVault) -> CPtrResult<raw::c_uchar> {
    let vault = unsafe { convert::borrow_in(vault) };
    let fun = || {
        let is_dirty = vault.is_dirty()?;
        Ok(convert::bool_out(is_dirty))
    };
    cresult(fun())
}
