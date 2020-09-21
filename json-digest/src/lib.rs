#![warn(missing_docs)]

//! This library provides some algorithms to calculate cryptographically secure digests of JSON documents.
//! Since JSON is an ambiguous serialization format, we also had to define a canonical deterministic subset
//! of all allowed documents. Order of keys in an object and Unicode normalization are well-defined in this
//! subset, making it suitable for hashing.
//!
//! ```
//! let data = serde_json::json!({
//!   "address": {
//!     "value": "6 Unter den Linden, Berlin, Germany",
//!     "nonce": "uN_FTaYe8JM-EZ8SU94kAOf0k0YvnhLcZgdpQ3BU9Ymbu"
//!   },
//!   "dateOfBirth": {
//!     "value": "16/02/2002",
//!     "nonce": "ufxkENKgXuf4yG50p6xpSyaQ8Gz7KsuqXid2yw533TUMK"
//!   },
//!   "placeOfBirth": {
//!     "city": "Berlin",
//!     "country": "Germany",
//!     "nonce": "ukhFsI4a6vIZEDUOBRxJmLroPEQ8FQCjJwbI-Z7bEocGo"
//!   },
//! });
//!
//! let digest = json_digest::digest_data(&data).unwrap();
//! assert_eq!(digest, "cjuQR3pDJeaiRv9oCZ-fBE7T8QWpUGfjP40sAXq0bLwr-8");
//!
//! let partial_digest = json_digest::selective_digest_data(&data, ".dateOfBirth").unwrap();
//! let expected_partial_digest = serde_json::json!({
//!   "address": "cjuvIf1PmPH_31JN5XqJ1xkcNDJyiw9zQ-7ansSB78gnt4",
//!   "dateOfBirth": {
//!     "nonce": "ufxkENKgXuf4yG50p6xpSyaQ8Gz7KsuqXid2yw533TUMK",
//!     "value":"16/02/2002"
//!   },
//!   "placeOfBirth": "cjub0Nxb0Kz0pI4bWCdSbaCutk1s5qieFT-ZmqUU1xcuAc"
//! });
//! assert_eq!(partial_digest, serde_json::to_string(&expected_partial_digest).unwrap());
//!
//! let digest_from_partial = json_digest::digest_json_str(&partial_digest).unwrap();
//! assert_eq!(digest, digest_from_partial);
//! ```

mod digest;
pub mod json_path;
mod nonce;

pub use digest::*;
pub use nonce::*;

use std::collections::HashMap;

use anyhow::{bail, ensure, Result};
use serde::{Deserialize, Serialize};
