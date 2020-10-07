pub mod vault_morpheus;

use super::*;

use iop_keyvault::{
    ed25519::{DidKind, Ed25519, Morpheus, MorpheusKind, MorpheusPrivateKey, MorpheusRoot},
    multicipher::{MKeyId, MPublicKey},
    Bip32Node, PublicKey as _, Seed,
};
use iop_vault::{BoundPlugin, PluginPrivate, PluginPublic, State, Vault, VaultPlugin};
