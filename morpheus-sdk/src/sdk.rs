use failure::{err_msg, Fallible};

use crate::io::{
    dist::did::{LedgerOperations, LedgerQueries},
    local::didvault::DidVault,
};

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

    pub fn vault(&self) -> Fallible<&V> {
        self.vault.as_ref().ok_or_else(|| err_msg("Vault is still uninitialized in Sdk Client"))
    }
    pub fn mut_vault(&mut self) -> Fallible<&mut V> {
        self.vault.as_mut().ok_or_else(|| err_msg("Vault is still uninitialized in Sdk Client"))
    }
    pub fn set_vault(&mut self, vault: V) -> Fallible<()> {
        if self.vault.is_some() {
            return Err(err_msg("Vault has already been initialized in Sdk Client"));
        }
        self.vault.replace(vault);
        Ok(())
    }

    pub fn ledger(&self) -> Fallible<&L> {
        self.ledger.as_ref().ok_or_else(|| err_msg("Ledger is still uninitialized in Sdk Client"))
    }
    pub fn mut_ledger(&mut self) -> Fallible<&mut L> {
        self.ledger.as_mut().ok_or_else(|| err_msg("Ledger is still uninitialized in Sdk Client"))
    }
    pub fn set_ledger(&mut self, ledger: L) -> Fallible<()> {
        if self.ledger.is_some() {
            return Err(err_msg("Ledger has already been initialized in Sdk Client"));
        }
        self.ledger.replace(ledger);
        Ok(())
    }
}
