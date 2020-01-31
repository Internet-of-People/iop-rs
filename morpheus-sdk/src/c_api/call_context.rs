use std::os::raw;
// use std::panic::catch_unwind; // TODO consider panic unwinding strategies

use failure::Fallible;

use super::convert;

#[repr(C)]
pub struct RequestId {
    _private_internal_layout: [u8; 0],
}
pub type Callback<T> = extern "C" fn(*mut RequestId, T) -> ();

pub struct CallContext<T> {
    id: *mut RequestId,
    success: Callback<T>,
    error: Callback<*const raw::c_char>,
}

impl<T: 'static> CallContext<T> {
    pub fn new(
        id: *mut RequestId, success: Callback<T>, error: Callback<*const raw::c_char>,
    ) -> Self {
        Self { id, success, error }
    }

    fn dispatch(self, result: Fallible<T>) {
        match result {
            Ok(val) => (self.success)(self.id, val),
            Err(err) => (self.error)(self.id, convert::string_out(err.to_string())),
        }
    }

    pub fn run(self, f: impl FnOnce() -> Fallible<T>) {
        self.dispatch(f())
    }
}
