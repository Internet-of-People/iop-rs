mod domain;
mod domain_name;
mod operations;
mod policy;
mod price;
mod principal;
mod signed;
mod state;

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
use std::ops;
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

pub type BlockHeight = u64;
pub type Nonce = u64;
