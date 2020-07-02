use super::*;

#[repr(C)]
pub struct CSlice<T> {
    first: *mut T,
    length: usize,
}

impl<T> CSlice<T> {
    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.first, self.length) }
    }
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.first, self.length) }
    }
}

impl<T> From<Box<[T]>> for CSlice<T> {
    fn from(mut slice: Box<[T]>) -> Self {
        let length = slice.len();
        let first = if length != 0 {
            let first = slice.as_mut_ptr();
            Box::into_raw(slice);
            first
        } else {
            std::ptr::null_mut()
        };
        Self { first, length }
    }
}

impl<T> From<Vec<T>> for CSlice<T> {
    fn from(vec: Vec<T>) -> Self {
        Self::from(vec.into_boxed_slice())
    }
}

impl From<Vec<String>> for CSlice<*mut raw::c_char> {
    fn from(src: Vec<String>) -> Self {
        let cptr_box_slice = src.into_iter().map(convert::string_out).collect::<Box<[_]>>();
        CSlice::from(cptr_box_slice)
    }
}
