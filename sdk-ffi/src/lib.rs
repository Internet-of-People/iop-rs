#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(non_snake_case)]

mod coeus;
mod crypto;
mod did;
mod ffi;
mod hydra;
mod jwt;
mod keyvault;
mod morpheus;
mod signed;
mod tx;
mod validation;
mod vault;

// use std::panic::catch_unwind; // TODO consider panic unwinding strategies
use std::os::raw;
use std::ptr::null_mut;
use std::str::FromStr;

use anyhow::{Context, Result};

use iop_keyvault::{
    ed25519::{MorpheusPrivateKey, MorpheusPublicKey},
    multicipher::*,
    secp256k1::*,
    Bip32Node, Bip32PublicNode, Bip44Key, Bip44PublicKey, Network, Networks, PrivateKey as _,
    PublicKey as _,
};
use iop_morpheus_core::{
    crypto::{jwt::*, sign::*},
    data::{claim::*, did::*, diddoc::*, present::*, validation::*},
};
use iop_morpheus_sdk::vault::{Plugin, Private, PrivateKind, Public, PublicKind};
use iop_vault::{BoundPlugin, Vault};
use json_digest::*;

// TODO consider killing usize type all around FFI
use crate::ffi::{convert, *};
