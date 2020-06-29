#![allow(clippy::not_unsafe_ptr_arg_deref)]

mod bip;
mod convert;
mod cres;
mod crypto;
mod hydra;
mod morpheus;
mod vault;

use std::os::raw;
use std::ptr::null;
// use std::panic::catch_unwind; // TODO consider panic unwinding strategies

use failure::Fallible;

use crate::convert::CSlice;
use crate::cres::*;
use iop_morpheus_core::crypto::hd::{BoundPlugin, Vault};
