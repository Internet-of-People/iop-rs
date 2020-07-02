use super::*;

pub(crate) unsafe fn borrow_in<'a, T>(value: *const T) -> &'a T {
    &*value
}

pub(crate) unsafe fn borrow_mut_in<'a, T>(value: *mut T) -> &'a mut T {
    &mut *value
}

pub(crate) unsafe fn str_in<'a>(s: *const raw::c_char) -> Fallible<&'a str> {
    let c_str = ffi::CStr::from_ptr(s);
    let s = c_str.to_str()?;
    Ok(s)
}

pub(crate) fn string_out(s: String) -> *mut raw::c_char {
    let c_str = ffi::CString::new(s).unwrap();
    c_str.into_raw()
}

// TODO this normally should be just a simple c_uchar,
//      but CallContext assumes a pointer on the C client side
pub(crate) fn bool_out(b: bool) -> *mut raw::c_uchar {
    let heap_byte = Box::new(b as u8);
    Box::into_raw(heap_byte)
}

pub(crate) fn move_out<T>(value: T) -> *mut T {
    Box::into_raw(Box::new(value))
}
