#![allow(clippy::new_without_default)] // WASM does not call default()

mod coeus;
mod doc;
mod morpheus;

pub use coeus::*;
pub use doc::*;
pub use morpheus::*;

pub use iop_keyvault_wasm::*;
pub use iop_proto_wasm::*;
pub use json_digest_wasm::*;

use serde::Serialize;
use wasm_bindgen::prelude::*;

use iop_coeus_node::{State as CoeusState, Version};
use iop_coeus_proto::*;
use iop_journal_proto::*;
use iop_morpheus_node::{StateHolder as MorpheusState, TransactionIdWithHeight};
use iop_morpheus_proto::{data::DidDocument, txtype::MorpheusAsset};
use json_digest_wasm::MapJsError;
