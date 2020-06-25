use std::os::raw;
// use std::panic::catch_unwind; // TODO consider panic unwinding strategies

use super::*;

#[repr(C)]
pub struct CPtrResult<T>(*mut CResult<T>);

impl From<Fallible<()>> for CPtrResult<raw::c_void> {
    fn from(result: Fallible<()>) -> Self {
        Self(convert::move_out(result.into()))
    }
}

impl<T> From<Fallible<*mut T>> for CPtrResult<T> {
    fn from(result: Fallible<*mut T>) -> Self {
        Self(convert::move_out(result.into()))
    }
}

#[repr(C)]
pub struct CResult<T> {
    success: *const T,
    error: *const raw::c_char,
}

impl<T> From<Fallible<*mut T>> for CResult<T> {
    fn from(result: Fallible<*mut T>) -> Self {
        match result {
            Ok(val) => CResult { success: val, error: null() },
            Err(err) => CResult { success: null(), error: convert::string_out(err.to_string()) },
        }
    }
}

impl<T> From<Fallible<()>> for CResult<T> {
    fn from(result: Fallible<()>) -> Self {
        match result {
            Ok(()) => CResult { success: null(), error: null() },
            Err(err) => CResult { success: null(), error: convert::string_out(err.to_string()) },
        }
    }
}
