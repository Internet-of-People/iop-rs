#![allow(clippy::not_unsafe_ptr_arg_deref)]

mod bip;
mod crypto;
mod did;
mod ffi;
mod hydra;
mod morpheus;
mod multicipher;
mod tx;
mod vault;

// use std::panic::catch_unwind; // TODO consider panic unwinding strategies
use std::os::raw;

use failure::Fallible;

use iop_keyvault::{multicipher::*, secp256k1::*, PrivateKey as _, PublicKey as _};
use iop_morpheus_core::{
    crypto::hd::{BoundPlugin, Vault},
    data::did::Did,
};

use crate::ffi::{convert, *};
