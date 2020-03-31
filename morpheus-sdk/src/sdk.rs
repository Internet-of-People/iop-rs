use failure::Fallible;

use crate::{
    client::Client,
    io::dist::did::{HydraDidLedger, /*FakeDidLedger, */ LedgerOperations, LedgerQueries},
    io::local::didvault::{DidVault, FilePersister, PersistentDidVault},
};
use iop_keyvault::Seed;
use iop_morpheus_core::{
    crypto::sign::Signed,
    data::{
        auth::Authentication,
        claim::{WitnessRequest, WitnessStatement},
        did::Did,
        diddoc::DidDocument,
        present::ClaimPresentation,
    },
    vault::InMemoryDidVault,
};

pub type SdkContext = Sdk<PersistentDidVault, HydraDidLedger>;

pub struct Sdk<V: DidVault, L: LedgerQueries + LedgerOperations> {
    pub client: Client<V, L>,
    pub reactor: tokio::runtime::Runtime,
}

impl<V: DidVault, L: LedgerQueries + LedgerOperations> Sdk<V, L> {
    pub fn new() -> Fallible<Self> {
        let reactor = tokio::runtime::Builder::new()
            .enable_all()
            .basic_scheduler()
            .build()
            .expect("Failed to initialize Tokio runtime");
        Ok(Self { client: Default::default(), reactor })
    }

    pub fn list_dids(&self) -> Fallible<Vec<Did>> {
        self.client.vault()?.dids()
    }

    pub fn create_did(&mut self) -> Fallible<Did> {
        let vault = self.client.mut_vault()?;
        self.reactor.block_on(async { Ok(vault.create(None).await?.did()) })
    }

    pub fn get_document(&mut self, did: &Did) -> Fallible<DidDocument> {
        let ledger = self.client.ledger()?;
        self.reactor.block_on(async { Ok(ledger.document(did).await?) })
    }

    pub fn sign_witness_request(
        &mut self, req: WitnessRequest, auth: &Authentication,
    ) -> Fallible<Signed<WitnessRequest>> {
        let vault = self.client.vault()?;
        self.reactor.block_on(async {
            let signer = vault.signer_by_auth(auth)?;
            signer.sign_witness_request(req).await
        })
    }

    pub fn sign_witness_statement(
        &mut self, stmt: WitnessStatement, auth: &Authentication,
    ) -> Fallible<Signed<WitnessStatement>> {
        let vault = self.client.vault()?;
        self.reactor.block_on(async {
            let signer = vault.signer_by_auth(auth)?;
            signer.sign_witness_statement(stmt).await
        })
    }

    pub fn sign_claim_presentation(
        &mut self, presentation: ClaimPresentation, auth: &Authentication,
    ) -> Fallible<Signed<ClaimPresentation>> {
        let vault = self.client.vault()?;
        self.reactor.block_on(async {
            let signer = vault.signer_by_auth(auth)?;
            signer.sign_claim_presentation(presentation).await
        })
    }

    pub fn close(self) -> Fallible<()> {
        Ok(())
    }
}

impl SdkContext {
    pub fn create_vault(&mut self, seed: &str, path: &str) -> Fallible<()> {
        let seed: Seed = Seed::from_bip39(seed)?;
        let mem_vault = InMemoryDidVault::new(seed);
        let file_persister = Box::new(FilePersister::new(&path));
        let mut persistent_vault = PersistentDidVault::new(mem_vault, file_persister);
        self.reactor.block_on(async { persistent_vault.save().await })?;
        self.client.set_vault(persistent_vault)
    }

    pub fn load_vault(&mut self, path: &str) -> Fallible<()> {
        let client = &mut self.client;
        self.reactor.block_on(async {
            let file_persister = Box::new(FilePersister::new(&path));
            let persistent_vault = PersistentDidVault::load(file_persister).await?;
            client.set_vault(persistent_vault)
        })
    }

    pub fn fake_ledger(&mut self) -> Fallible<()> {
        todo!();
        // self.client.set_ledger(FakeDidLedger::new())?;
        // Ok(())
    }

    pub fn real_ledger(&mut self, url: &str) -> Fallible<()> {
        self.client.set_ledger(HydraDidLedger::new(url))?;
        Ok(())
        // Err(err_msg("Not implemented yet"))
    }
}
