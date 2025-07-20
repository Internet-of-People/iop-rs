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
mod validation;
mod vault;

// use std::panic::catch_unwind; // TODO consider panic unwinding strategies
use std::os::raw;
use std::str::FromStr;

use anyhow::{Result, ensure};

use iop_hydra_proto::txtype::{
    Aip29Transaction, CommonTransactionFields, OptionalTransactionFields,
};
use iop_journal_proto::BlockHeight;
use iop_keyvault::{
    Bip32Node, Bip32PublicNode, Bip44Key, Bip44PublicKey, Network, Networks, PrivateKey as _,
    PublicKey as _,
    ed25519::{DidKind, MorpheusPrivateKey, MorpheusPublicKey},
    multicipher::*,
    secp256k1::*,
};
use iop_morpheus_proto::{
    crypto::{jwt::*, sign::*},
    data::*,
};
use iop_morpheus_sdk::vault::{
    Plugin as MorpheusPlugin, Private as MorpheusPrivate, PrivateKind as MorpheusPrivateKind,
    Public as MorpheusPublic, PublicKind as MorpheusPublicKind,
};
use iop_vault::{BoundPlugin, Vault};
use json_digest::*;

// TODO consider killing usize type all around FFI
use crate::ffi::{convert, *};
