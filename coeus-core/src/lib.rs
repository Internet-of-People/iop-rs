mod asset;
mod domain;
mod domain_name;
mod operations;
mod policy;
mod price;
mod principal;
mod signed;
mod state;

pub use asset::*;
pub use domain::*;
pub use domain_name::*;
pub use operations::*;
pub use policy::*;
pub use price::*;
pub use principal::*;
pub use signed::*;
pub use state::*;

use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::hash::Hash;
use std::str::FromStr;

use anyhow::{bail, ensure, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use valico::json_schema;

use iop_keyvault::{
    multicipher::{MPrivateKey, MPublicKey, MSignature},
    PrivateKey as _, PublicKey as _,
};
#[cfg(feature = "did")]
use iop_morpheus_core::data::did::Did;
use json_digest::canonical_json;

// TODO move all blockchain-related types to hydra-proto after adding typetags to Asset and TransactionType.
/// State identifier of a decentralized ledger, usually represented as a sequence number of blocks.
pub type BlockHeight = u32;
/// Duration (e.g. a year) expressed as an expected number of blocks on the ledger, approximating the duration.  
pub type BlockCount = u32;
/// A unique value attached to requests as protection from replay attacks.
pub type Nonce = u64;
