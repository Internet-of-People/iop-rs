#![warn(missing_docs)]

//! This library provides some algorithms to calculate cryptographically secure digests of JSON documents.
//! Since JSON is an ambiguous serialization format, we also had to define a canonical deterministic subset
//! of all allowed documents. Order of keys in an object and Unicode normalization are well-defined in this
//! subset, making it suitable for hashing.

mod digest;
pub mod json_path;
mod nonce;

pub use digest::*;
pub use nonce::*;

use std::collections::HashMap;

use anyhow::{bail, ensure, Result};
use serde::{Deserialize, Serialize};
