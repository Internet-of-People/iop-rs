use super::*;

use crate::io::{
    dist::did::{LedgerOperations, LedgerQueries},
    local::didvault::DidVault,
};
use anyhow::Context;

pub struct Client<V: DidVault, L: LedgerQueries + LedgerOperations> {
    vault: Option<V>,
    ledger: Option<L>,
}

impl<V: DidVault, L: LedgerQueries + LedgerOperations> Default for Client<V, L> {
    fn default() -> Self {
        Self { vault: None, ledger: None }
    }
}

impl<V: DidVault, L: LedgerQueries + LedgerOperations> Client<V, L> {
    pub fn new(vault: V, ledger: L) -> Self {
        Self { vault: Some(vault), ledger: Some(ledger) }
    }

    pub fn vault(&self) -> Result<&V> {
        self.vault.as_ref().with_context(|| "Vault is still uninitialized in Sdk Client")
    }
    pub fn mut_vault(&mut self) -> Result<&mut V> {
        self.vault.as_mut().with_context(|| "Vault is still uninitialized in Sdk Client")
    }
    pub fn set_vault(&mut self, vault: V) -> Result<()> {
        if self.vault.is_some() {
            return Err(anyhow!("Vault has already been initialized in Sdk Client"));
        }
        self.vault.replace(vault);
        Ok(())
    }

    pub fn ledger(&self) -> Result<&L> {
        self.ledger.as_ref().with_context(|| "Ledger is still uninitialized in Sdk Client")
    }
    pub fn mut_ledger(&mut self) -> Result<&mut L> {
        self.ledger.as_mut().with_context(|| "Ledger is still uninitialized in Sdk Client")
    }
    pub fn set_ledger(&mut self, ledger: L) -> Result<()> {
        if self.ledger.is_some() {
            return Err(anyhow!("Ledger has already been initialized in Sdk Client"));
        }
        self.ledger.replace(ledger);
        Ok(())
    }
}
