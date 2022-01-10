#![allow(clippy::new_without_default)] // WASM does not call default()

mod coeus;
mod did;
mod jwt;
mod sign;

pub use coeus::*;
pub use did::*;
pub use jwt::*;
pub use sign::*;

// imports from standard library

// imports from 3rd party crates

use anyhow::Result;
use wasm_bindgen::prelude::*;

// imports from own crates

use iop_coeus_proto::*;
use iop_journal_proto::*;
use iop_keyvault_wasm::*;
use iop_morpheus_proto::{
    crypto::{
        jwt::{JwtBuilder, JwtParser},
        sign::{Signable, Signed},
    },
    data::{Did, ValidationIssue, ValidationResult},
};

pub use iop_keyvault_wasm::*;
