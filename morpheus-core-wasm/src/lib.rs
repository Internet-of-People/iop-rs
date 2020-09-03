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

// imports from 3rd party crates

use failure::ResultExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;

// imports from own crates

use iop_keyvault::{multicipher::*, Networks, PublicKey as _};
use iop_keyvault_wasm::*;
use iop_morpheus_core::{
    crypto::{
        hd::{hydra as hd_hydra, morpheus as hd_morpheus, BoundPlugin, Vault, VaultPlugin},
        json_digest::{canonical_json, selective_digest_json},
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
    hydra::TransactionData as HydraTransactionData,
};
