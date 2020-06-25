use std::os::raw;
// use std::panic::catch_unwind; // TODO consider panic unwinding strategies

use super::*;

#[repr(C)]
pub struct CResult<T> {
    success: *const T,
    error: *const raw::c_char,
}

#[repr(C)]
pub struct CPtrResult<T>(*mut CResult<T>);

impl<T: 'static> CPtrResult<T> {
    pub fn run(f: impl FnOnce() -> Fallible<*mut T>) -> Self {
        let cptr = f().into();
        Self(convert::move_out(cptr))
    }

    pub fn run_void(f: impl FnOnce() -> Fallible<()>) -> Self {
        let cptr = f().map(|()| null()).into();
        Self(convert::move_out(cptr))
    }
}

impl<T> From<Fallible<*const T>> for CResult<T> {
    fn from(result: Fallible<*const T>) -> Self {
        match result {
            Ok(val) => CResult { success: val, error: null() },
            Err(err) => CResult { success: null(), error: convert::string_out(err.to_string()) },
        }
    }
}

impl<T> From<Fallible<*mut T>> for CResult<T> {
    fn from(result: Fallible<*mut T>) -> Self {
        match result {
            Ok(val) => CResult { success: val, error: null() },
            Err(err) => CResult { success: null(), error: convert::string_out(err.to_string()) },
        }
    }
}
