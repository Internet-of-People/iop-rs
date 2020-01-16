use failure::Fallible;

use crate::data::{did::Did, diddoc::Right};
use crate::io::{
    dist::did::{LedgerOperations, LedgerQueries},
    local::didvault::{DidVault, PersistentDidVault},
};

pub struct Client<V: DidVault, L: LedgerQueries + LedgerOperations> {
    vault: V,
    ledger: L,
}

impl<V: DidVault, L: LedgerQueries + LedgerOperations> Client<V, L> {
    pub fn new(vault: V, ledger: L) -> Self {
        Self { vault, ledger }
    }

    pub fn vault(&self) -> &V {
        &self.vault
    }
    pub fn ledger(&self) -> &L {
        &self.ledger
    }
}
