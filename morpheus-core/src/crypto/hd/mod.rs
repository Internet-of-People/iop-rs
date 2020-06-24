// sub-modules

mod classic;
pub mod did;
pub mod hydra;
pub mod morpheus;
mod state;
mod vault;

// exports

pub use classic::*;
pub use state::*;
pub use vault::*;

// imports from standard library

use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;

// imports from 3rd party crates

use failure::{bail, ensure, err_msg, format_err, Fallible};
use log::*;
use serde::{Deserialize, Serialize};

// imports from own crates

use iop_keyvault::{
    ed25519::*, encrypt::*, multicipher::*, secp256k1::*, Bip32Node, Bip32PublicNode, Bip39, Bip44,
    Bip44Account, Bip44AccountPath, Bip44Key, Bip44PublicAccount, Bip44PublicKey,
    ExtendedPrivateKey as _xsk, ExtendedPublicKey as _xpk, KeyDerivationCrypto as _kd, Network,
    Networks, PublicKey as _pk, Seed, BIP43_PURPOSE_MERCURY,
};

// imports from super

use super::*;
