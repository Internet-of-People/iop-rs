// sub-modules

mod did;
mod hydra;
mod json;
mod jwt;
mod morpheus;
mod sign;
mod vault;

// exports

pub use did::*;
pub use hydra::*;
pub use json::*;
pub use jwt::*;
pub use morpheus::*;
pub use sign::*;
pub use vault::*;

// imports from standard library

use std::str::FromStr;

// imports from 3rd party crates

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

// imports from own crates

use iop_hydra_proto::{
    txtype::{hyd_core, Aip29Transaction, CommonTransactionFields, OptionalTransactionFields},
    TransactionData as HydraTransactionData,
};
use iop_hydra_sdk::vault as hd_hydra;
use iop_keyvault::{
    multicipher::*,
    secp256k1::{Secp256k1, SecpPublicKey},
    Network, Networks, PublicKey as _,
};
use iop_keyvault_wasm::*;
use iop_morpheus_core::{
    crypto::{
        jwt::{JwtBuilder, JwtParser},
        sign::{PrivateKeySigner, Signable, Signed, SyncMorpheusSigner},
    },
    data::{
        claim::{WitnessRequest, WitnessStatement},
        did::Did,
        diddoc::BlockHeight,
        present::ClaimPresentation,
        validation::{ValidationIssue, ValidationResult},
    },
};
use iop_morpheus_sdk::vault as hd_morpheus;
use iop_vault::{BoundPlugin, Vault, VaultPlugin};
use json_digest::*;
