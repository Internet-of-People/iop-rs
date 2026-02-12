//! Core type definitions for the `did:mosaic` DID method.
//!
//! This crate defines the data types used by the Mosaic Trust Network DID system,
//! which is derived from the IOP Morpheus SSI Stack and adapted for the
//! Substrate/Polkadot ecosystem.
//!
//! # Architecture
//!
//! The `did:mosaic` method preserves the battle-tested semantics of IOP Morpheus
//! while introducing:
//! - BLAKE3-256 for DID identifier derivation (faster, modern)
//! - SCALE codec encoding for Substrate compatibility
//! - Expanded operations set (Phase 1: 5 core + BeforeProof)
//! - Expanded rights model (6 rights vs original 2)
//!
//! # Anti-Censorship Design
//!
//! Any account can submit operations for any DID. Authorization is verified via
//! cryptographic signatures inside operations, not transaction origin. This prevents
//! node censorship of specific DIDs.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod did;
pub mod document;
pub mod operation;
pub mod signature;

pub use did::*;
pub use document::*;
pub use operation::*;
pub use signature::*;
