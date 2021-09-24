// use std::panic::catch_unwind; // TODO consider panic unwinding strategies

use super::*;

pub type CPtrResult<T> = *mut CResult<T>;

#[repr(C)]
pub struct CResult<T> {
    success: *mut T,
    error: *mut raw::c_char,
}

pub(crate) fn cresult_void(result: Result<()>) -> *mut CResult<raw::c_void> {
    let cres = match result {
        Ok(()) => CResult { success: null_mut(), error: null_mut() },
        Err(err) => CResult { success: null_mut(), error: convert::string_out(err.to_string()) },
    };
    convert::move_out(cres)
}

pub(crate) fn cresult<T>(result: Result<*mut T>) -> *mut CResult<T> {
    let cres = match result {
        Ok(val) => CResult { success: val, error: null_mut() },
        Err(err) => CResult { success: null_mut(), error: convert::string_out(err.to_string()) },
    };
    convert::move_out(cres)
}
