pub mod crypto;
pub mod data;
pub mod txtype;

use std::ops::Deref;

use anyhow::{Result, anyhow, bail, ensure};
use serde::{Deserialize, Serialize};

use iop_journal_proto::{BlockHeight, serializer};
use iop_keyvault::{
    PublicKey,
    multicipher::{MPublicKey, MSignature},
};
use json_digest::{Nonce264, canonical_json, default_hasher, digest_data};
