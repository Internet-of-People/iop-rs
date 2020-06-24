#![allow(clippy::not_unsafe_ptr_arg_deref)]

mod bip;
mod call_context;
mod convert;
mod crypto;
mod hydra;
mod morpheus;
mod vault;

use std::os::raw;
// use std::panic::catch_unwind; // TODO consider panic unwinding strategies

use failure::Fallible;

use self::call_context::CallContext;
use self::convert::RawSlice;
