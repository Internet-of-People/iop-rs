mod plugin;
mod private;
mod private_key;
mod private_kind;
mod public;
mod public_kind;

use iop_keyvault::{
    ed25519::{MorpheusPrivateKey, MorpheusPublicKey},
    multicipher::*,
    PublicKey as _,
};
use iop_morpheus_core::crypto::hd::morpheus::{Plugin, Private, PrivateKind, Public, PublicKind};

use plugin::CMorpheusPlugin;

use super::*;
