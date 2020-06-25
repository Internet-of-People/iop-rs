use std::os::raw;
// use std::panic::catch_unwind; // TODO consider panic unwinding strategies

use super::*;

pub type CPtrResult<T> = *mut CResult<T>;

#[repr(C)]
pub struct CResult<T> {
    success: *const T,
    error: *const raw::c_char,
}

pub(crate) fn cresult_void(result: Fallible<()>) -> *mut CResult<raw::c_void> {
    let cres = match result {
        Ok(()) => CResult { success: null(), error: null() },
        Err(err) => CResult { success: null(), error: convert::string_out(err.to_string()) },
    };
    convert::move_out(cres)
}

pub(crate) fn cresult<T>(result: Fallible<*mut T>) -> *mut CResult<T> {
    let cres = match result {
        Ok(val) => CResult { success: val, error: null() },
        Err(err) => CResult { success: null(), error: convert::string_out(err.to_string()) },
    };
    convert::move_out(cres)
}
