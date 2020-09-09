use super::*;

use std::fs::File;
use std::path::{Path, PathBuf};

use crate::io::local::signer::{MorpheusSigner, SyncAdapter};
use iop_keyvault::multicipher::MKeyId;
use iop_morpheus_core::{
    crypto::hd::{did::*, HdRecord, Label},
    data::{auth::Authentication, did::*},
};

#[async_trait(?Send)]
pub trait DidVault {
    fn key_ids(&self) -> Result<Vec<MKeyId>>;
    fn dids(&self) -> Result<Vec<Did>>;

    fn get_active(&self) -> Result<Option<Did>>;
    async fn set_active(&mut self, did: &Did) -> Result<()>;

    fn record_by_auth(&self, auth: &Authentication) -> Result<HdRecord>;
    // async fn restore_id(&mut self, did: &Did) -> Result<()>;
    fn signer_by_auth(&self, auth: &Authentication) -> Result<Box<dyn MorpheusSigner>>;

    async fn create(&mut self, label: Option<Label>) -> Result<HdRecord>;
    async fn update(&mut self, record: HdRecord) -> Result<()>;
}

pub struct PersistentDidVault {
    in_memory_vault: InMemoryDidVault,
    persister: Box<dyn Persister>,
}

impl PersistentDidVault {
    pub fn new(in_memory_vault: InMemoryDidVault, persister: Box<dyn Persister>) -> Self {
        Self { in_memory_vault, persister }
    }

    pub async fn load(persister: Box<dyn Persister>) -> Result<Self> {
        let reader = persister.reader()?;
        let vault: InMemoryDidVault = serde_json::from_reader(reader)?;
        //let vault: Self = bincode::deserialize_from(vault_file)?;
        vault.verify_integrity()?;
        Ok(Self::new(vault, persister))
    }

    pub async fn save(&mut self) -> Result<()> {
        let writer = self.persister.writer()?;
        serde_json::to_writer_pretty(writer, &self.in_memory_vault)?;
        //bincode::serialize_into(vault_file, self)?;
        Ok(())
    }
}

#[async_trait(?Send)]
impl DidVault for PersistentDidVault {
    fn key_ids(&self) -> Result<Vec<MKeyId>> {
        self.in_memory_vault.key_ids()
    }
    fn dids(&self) -> Result<Vec<Did>> {
        self.in_memory_vault.dids()
    }

    fn get_active(&self) -> Result<Option<Did>> {
        self.in_memory_vault.get_active()
    }

    async fn set_active(&mut self, did: &Did) -> Result<()> {
        self.in_memory_vault.set_active(did)?;
        self.save().await
    }

    fn record_by_auth(&self, auth: &Authentication) -> Result<HdRecord> {
        self.in_memory_vault.record_by_auth(auth)
    }

    fn signer_by_auth(&self, auth: &Authentication) -> Result<Box<dyn MorpheusSigner>> {
        let sync_signer = self.in_memory_vault.signer_by_auth(auth)?;
        Ok(Box::new(SyncAdapter::new(sync_signer)))
    }

    async fn create(&mut self, label_opt: Option<Label>) -> Result<HdRecord> {
        let result = self.in_memory_vault.create(label_opt)?;
        self.save().await?;
        Ok(result)
    }

    async fn update(&mut self, record: HdRecord) -> Result<()> {
        self.in_memory_vault.update(record)?;
        self.save().await
    }
}

pub trait Persister {
    fn reader(&self) -> Result<Box<dyn std::io::Read>>;
    fn writer(&self) -> Result<Box<dyn std::io::Write>>;
}

pub struct FilePersister {
    path: PathBuf,
}

impl FilePersister {
    pub fn new(path: &impl AsRef<Path>) -> Self {
        Self { path: path.as_ref().to_owned() }
    }
}

impl Persister for FilePersister {
    fn reader(&self) -> Result<Box<dyn std::io::Read>> {
        debug!("Loading DidVault from {:?}", self.path);
        let vault_file = File::open(&self.path)?;
        Ok(Box::new(vault_file))
    }

    fn writer(&self) -> Result<Box<dyn std::io::Write>> {
        debug!("Saving profile vault to persist its state");
        if let Some(vault_dir) = self.path.parent() {
            debug!("Recursively Creating directory {:?}", vault_dir);
            std::fs::create_dir_all(vault_dir)?;
        }

        let vault_file = File::create(&self.path)?;
        Ok(Box::new(vault_file))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use iop_keyvault::{Bip39, Seed};

    fn in_memory_vault_instance() -> Result<InMemoryDidVault> {
        let seed = Bip39::new().phrase(Seed::DEMO_PHRASE)?.password(Seed::PASSWORD);
        Ok(InMemoryDidVault::new(seed))
    }

    async fn test_vault<T: DidVault>(vault: &mut T) -> Result<()> {
        assert_eq!(vault.dids()?, vec![]);

        let record1 = vault.create(None).await?;
        assert_eq!(vault.dids()?, vec![record1.did()]);
        assert_eq!(record1.bip32_idx(), 0);
        assert_eq!(record1.did().to_string(), "did:morpheus:ezbeWGSY2dqcUBqT8K7R14xr");

        let record2 = vault.create(None).await?;
        assert_eq!(vault.dids()?, vec![record1.did(), record2.did()]);
        assert_eq!(record2.bip32_idx(), 1);
        assert_eq!(record2.did().to_string(), "did:morpheus:ez25N5WZ1Q6TQpgpyYgiu9gTX");

        Ok(())
    }

    #[tokio::test]
    async fn persistent_vault() -> Result<()> {
        let tmp_dir = tempfile::tempdir()?.into_path();
        let tmp_file = tmp_dir.join("morpheus-testvault.dat");
        let file_persister = Box::new(FilePersister::new(&tmp_file));
        let file_persister_clone = Box::new(FilePersister::new(&tmp_file));
        //let tmp_file_str = tmp_file.into_os_string().into_string()?;
        let mem_vault = in_memory_vault_instance()?;
        let mut persistent_vault = PersistentDidVault::new(mem_vault, file_persister);
        test_vault(&mut persistent_vault).await?;

        let mem_vault = &persistent_vault.in_memory_vault;
        let loaded_vault = PersistentDidVault::load(file_persister_clone).await?;
        let loaded_mem_vault = &loaded_vault.in_memory_vault;
        loaded_mem_vault.verify_integrity()?;
        assert_eq!(loaded_mem_vault.dids()?, mem_vault.dids()?);
        assert_eq!(loaded_mem_vault.get_active()?, mem_vault.get_active()?);
        Ok(())
    }
}
