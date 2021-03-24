mod hydra;
mod morpheus;
mod vault;

pub use hydra::*;
pub use morpheus::*;
pub use vault::*;

// imports from standard library

use std::str::FromStr;

// imports from 3rd party crates

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
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
use iop_morpheus_proto::{
    crypto::sign::{PrivateKeySigner, Signable, Signed, SyncMorpheusSigner},
    data::{
        auth::Authentication,
        claim::{WitnessRequest, WitnessStatement},
        did::Did,
        present::ClaimPresentation,
    },
};
use iop_morpheus_sdk::vault as hd_morpheus;
use iop_vault::{BoundPlugin, Vault, VaultPlugin};

pub use iop_keyvault_wasm::*;
pub use iop_proto_wasm::*;
pub use json_digest_wasm::*;
