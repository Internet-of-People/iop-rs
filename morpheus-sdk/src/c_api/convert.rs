use std::ffi;
use std::os::raw;

use failure::Fallible;

pub fn str_in<'a>(s: *const raw::c_char) -> Fallible<&'a str> {
    let c_str = unsafe { ffi::CStr::from_ptr(s) };
    let s = c_str.to_str()?;
    Ok(s)
}

pub fn string_out(s: String) -> *mut raw::c_char {
    let c_str = ffi::CString::new(s).unwrap();
    c_str.into_raw()
}

pub fn bool_out(b: bool) -> *mut raw::c_uchar {
    let heap_byte = Box::new(b as u8);
    Box::into_raw(heap_byte)
}

#[repr(C)]
pub struct RawSlice<T> {
    first: *mut T,
    length: usize,
}

impl<T> From<&mut [T]> for RawSlice<T> {
    fn from(slice: &mut [T]) -> Self {
        let first = slice.as_mut_ptr();
        let length = slice.len();
        Self { first, length }
    }
}
