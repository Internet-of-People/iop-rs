use std::ffi;
use std::os::raw;

use failure::Fallible;

// pub unsafe fn borrow_in<'a, T>(value: *const T) -> &'a T {
//     &*value
// }

pub unsafe fn borrow_mut_in<'a, T>(value: *mut T) -> &'a mut T {
    &mut *value
}

pub fn str_in<'a>(s: *const raw::c_char) -> Fallible<&'a str> {
    let c_str = unsafe { ffi::CStr::from_ptr(s) };
    let s = c_str.to_str()?;
    Ok(s)
}

pub fn string_out(s: String) -> *mut raw::c_char {
    let c_str = ffi::CString::new(s).unwrap();
    c_str.into_raw()
}

// TODO this normally should be just a simple c_uchar,
//      figure out why that version crashes with Dart
pub fn bool_out(b: bool) -> *mut raw::c_uchar {
    let heap_byte = Box::new(b as u8);
    Box::into_raw(heap_byte)
}

pub fn move_out<T>(value: T) -> *mut T {
    Box::into_raw(Box::new(value))
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

impl From<Vec<String>> for RawSlice<*mut raw::c_char> {
    fn from(src: Vec<String>) -> Self {
        let cptr_box_slice = src.into_iter().map(string_out).collect::<Box<[_]>>();
        let raw_box_slice = Box::into_raw(cptr_box_slice);
        unsafe { &mut *raw_box_slice }.into()
    }
}