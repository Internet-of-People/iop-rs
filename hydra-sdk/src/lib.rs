pub mod vault;

// imports from standard library

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

// imports from 3rd party crates

use anyhow::{bail, ensure, Context, Result};
//use log::*;
use serde::{Deserialize, Serialize};

// imports from own crates

use iop_keyvault::{
    secp256k1::{Secp256k1, SecpPrivateKey, SecpPublicKey},
    Bip32Node, Bip32PublicNode, Bip44, Bip44Account, Bip44AccountPath, Bip44Key,
    Bip44PublicAccount, Bip44PublicKey, Network, Networks, PrivateKey as _, Seed,
};
use iop_morpheus_core::hydra::TransactionData;
use iop_vault::{BoundPlugin, PluginPrivate, PluginPublic, State};
