use failure::Fallible;

use crate::data::{did::Did, diddoc::Right};
use crate::io::{
    dist::did::{DidDocumentLedgerQueries, HydraDidLedger},
    local::didvault::{DidVault, PersistentDidVault},
};

pub struct Client<V: DidVault, L: DidDocumentLedgerQueries> {
    vault: V,
    ledger: L,
}

impl<V: DidVault, L: DidDocumentLedgerQueries> Client<V, L> {
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
