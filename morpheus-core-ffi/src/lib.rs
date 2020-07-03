#![allow(clippy::not_unsafe_ptr_arg_deref)]

mod bip39;
mod bip44;
mod crypto;
mod did;
mod ffi;
mod hydra;
mod morpheus;
mod multicipher;
mod secp;
mod tx;
mod vault;

// use std::panic::catch_unwind; // TODO consider panic unwinding strategies
use std::os::raw;
use std::str::FromStr;

use failure::Fallible;

use iop_keyvault::{
    multicipher::*, secp256k1::*, Bip44Key, Bip44PublicKey, PrivateKey as _, PublicKey as _,
};
use iop_morpheus_core::{
    crypto::hd::{BoundPlugin, Vault},
    data::did::Did,
};

use crate::ffi::{convert, *};
