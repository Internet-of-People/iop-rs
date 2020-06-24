use std::os::raw;
// use std::panic::catch_unwind; // TODO consider panic unwinding strategies

use failure::Fallible;

use super::convert;

#[repr(C)]
pub struct CallContext<T> {
    success: T,
    error: *mut raw::c_char,
}

impl<T: 'static> CallContext<T> {
    fn dispatch(&mut self, result: Fallible<T>) {
        match result {
            Ok(val) => self.success = val,
            Err(err) => self.error = convert::string_out(err.to_string()),
        }
    }

    pub fn run(&mut self, f: impl FnOnce() -> Fallible<T>) {
        self.dispatch(f())
    }
}
