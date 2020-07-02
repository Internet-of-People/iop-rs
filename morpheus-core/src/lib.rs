pub mod crypto;
pub mod data;
pub mod hydra;
pub mod util;

use failure::Fallible;
use serde::Serialize;

use data::{auth::Authentication, did::Did};
