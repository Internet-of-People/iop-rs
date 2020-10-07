pub mod classic;
pub mod client;
pub mod did;
pub mod io;
pub mod sdk;

use std::collections::HashMap;

use anyhow::{anyhow, bail, ensure, Context, Result};
use async_trait::async_trait;
use log::*;
use serde::{Deserialize, Serialize};

use crate::classic::*;
use crate::did::*;
use iop_keyvault::{
    ed25519::{Ed25519, EdExtPrivateKey},
    multicipher::{MKeyId, MPrivateKey, MPublicKey},
    ExtendedPrivateKey, ExtendedPublicKey, KeyDerivationCrypto, PublicKey as _, Seed,
    BIP43_PURPOSE_MERCURY,
};
use iop_morpheus_core::{
    crypto::sign::PrivateKeySigner,
    data::{auth::Authentication, did::*},
};
