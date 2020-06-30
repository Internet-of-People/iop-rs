pub(crate) mod convert;
mod cres;

use super::*;

use std::ffi;
use std::ptr::null;

pub(crate) use {convert::CSlice, cres::*};
