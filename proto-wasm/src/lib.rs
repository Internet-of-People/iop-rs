#![allow(clippy::new_without_default)] // WASM does not call default()
#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

//! This library defines WASM wrappers for objects used both by clients in the SDK and the blockchain node implementation.

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
use serde_wasm_bindgen::*;
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
