pub(crate) mod convert;
mod cres;
mod cslice;

use super::*;

use std::ffi;
use std::ptr::null;

pub(crate) use {cres::*, cslice::*};

pub(crate) fn delete<T>(t: *mut T) {
    let tbox_opt = unsafe { convert::move_in(t) };
    if let Some(tbox) = tbox_opt {
        drop(tbox); // NOTE redundant, but clearer than let _t = ...;
    }
}
