#![allow(clippy::not_unsafe_ptr_arg_deref)]

mod bip;
mod crypto;
mod ffi;
mod hydra;
mod morpheus;
mod vault;

// use std::panic::catch_unwind; // TODO consider panic unwinding strategies
use std::os::raw;

use failure::Fallible;

use crate::ffi::{convert, *};
use iop_morpheus_core::crypto::hd::{BoundPlugin, Vault};
