use std::fs::File;
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use failure::{ensure, format_err, Fallible};
use log::*;
use serde::{Deserialize, Serialize};

use crate::{
    crypto::sign::{PrivateKeySigner, Signer},
    data::{auth::Authentication, did::*},
};
use keyvault::{
    ed25519::{Ed25519, EdExtPrivateKey},
    multicipher, ExtendedPrivateKey, ExtendedPublicKey, KeyDerivationCrypto, PublicKey, Seed,
    BIP43_PURPOSE_MERCURY,
};

pub type Label = String;
pub type Metadata = String;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DidVaultRecord {
    bip32_idx: i32,
    public_key: multicipher::MPublicKey,
    label: Label,
    metadata: Metadata,
    // document: DidDocument,
    // #[serde(ignore)]
    // version: usize,
}

impl DidVaultRecord {
    fn new(bip32_idx: i32, pubkey: multicipher::MPublicKey, label: Label) -> Self {
        Self { bip32_idx, public_key: pubkey, label, metadata: Default::default() }
        // version: 0
        // document: DidDocument {}
    }

    pub fn bip32_idx(&self) -> i32 {
        self.bip32_idx
    }
    pub fn public_key(&self) -> multicipher::MPublicKey {
        self.public_key.to_owned()
    }
    pub fn key_id(&self) -> multicipher::MKeyId {
        self.public_key.key_id()
    }
    pub fn did(&self) -> Did {
        self.key_id().into()
    }
    pub fn label(&self) -> Label {
        self.label.to_owned()
    }
    pub fn set_label(&mut self, label: Label) {
        self.label = label;
    }
    pub fn metadata(&self) -> Metadata {
        self.metadata.to_owned()
    }
    pub fn set_metadata(&mut self, metadata: Metadata) {
        self.metadata = metadata;
    }
    // pub fn document(&self) -> DidDocument { self.document.to_owned() }
}

#[async_trait(?Send)]
pub trait DidVault {
    fn dids(&self) -> Fallible<Vec<Did>>;

    fn get_active(&self) -> Fallible<Option<Did>>;
    async fn set_active(&mut self, did: &Did) -> Fallible<()>;

    fn record_by_auth(&self, auth: &Authentication) -> Fallible<DidVaultRecord>;
    // async fn restore_id(&mut self, did: &Did) -> Fallible<()>;
    fn signer_by_auth(&self, auth: &Authentication) -> Fallible<Box<dyn Signer>>;

    async fn create(&mut self, label: Option<Label>) -> Fallible<DidVaultRecord>;
    async fn update(&mut self, record: DidVaultRecord) -> Fallible<()>;
}

// TODO On the long term, this architecture should be completely different.
//      Now it's PersistentDidVault( InMemoryDidVault(Seed) )
//      Should be something like the following on the long term
//      Vault(Seed or MasterKey) <-> Dids (subtree)
//                               <-> MercuryAddresses (another subtree)
#[derive(Debug, Deserialize, Serialize)]
pub struct InMemoryDidVault {
    seed: Seed,
    // TODO remove redundancy of next_idx and derive it from records.len() instead
    //      or decide to use sparse representation of records instead
    next_idx: usize,
    active_idx: Option<usize>,
    records: Vec<DidVaultRecord>,
}

impl InMemoryDidVault {
    pub fn new(seed: Seed) -> Self {
        Self { seed, next_idx: Default::default(), active_idx: Default::default(), records: vec![] }
    }

    // TODO this should not depend on Ed25519, should return something like MExtPrivateKey instead
    pub fn morpheus_xsk(&self) -> Fallible<EdExtPrivateKey> {
        let master = Ed25519::master(&self.seed);
        // TODO BIP43_PURPOSE_MORPHEUS
        let morpheus_xsk = master.derive_hardened_child(BIP43_PURPOSE_MERCURY)?;
        Ok(morpheus_xsk)
    }

    fn public_key(&self, idx: i32) -> Fallible<multicipher::MPublicKey> {
        let did_xsk = self.morpheus_xsk()?.derive_hardened_child(idx)?;
        let key = did_xsk.neuter().as_public_key();
        Ok(key.into())
    }

    fn private_key(&self, idx: i32) -> Fallible<multicipher::MPrivateKey> {
        let did_xsk = self.morpheus_xsk()?.derive_hardened_child(idx)?;
        let key = did_xsk.as_private_key();
        Ok(key.into())
    }

    fn index_of_did(&self, did: &Did) -> Option<usize> {
        let matches_it = self
            .records
            .iter()
            .enumerate()
            .filter(|(_idx, rec)| rec.public_key.validate_id(&did.default_key_id()));
        matches_it.map(|(idx, _rec)| idx).next()
    }

    fn did_by_label(&self, label: &Label) -> Fallible<Did> {
        self.records
            .iter()
            .filter_map(|rec| if rec.label == *label { Some(rec.did()) } else { None })
            .next()
            .ok_or_else(|| format_err!("Label {} is not found in vault", label))
    }
}

#[async_trait(?Send)]
impl DidVault for InMemoryDidVault {
    fn dids(&self) -> Fallible<Vec<Did>> {
        Ok(self.records.iter().map(|rec| rec.did()).collect())
    }

    fn get_active(&self) -> Fallible<Option<Did>> {
        if let Some(idx) = self.active_idx {
            ensure!(idx < self.records.len(), "Implementation error: invalid active_idx");
        }
        Ok(self.active_idx.map(|idx| self.records[idx].did()))
    }

    async fn set_active(&mut self, did: &Did) -> Fallible<()> {
        let idx = self
            .index_of_did(did)
            .ok_or_else(|| format_err!("Vault does not contain DID {}", did))?;
        self.active_idx = Some(idx);
        Ok(())
    }

    fn record_by_auth(&self, auth: &Authentication) -> Fallible<DidVaultRecord> {
        let rec_opt = self
            .records
            .iter()
            .filter(|rec| match auth {
                Authentication::KeyId(id) => rec.public_key.validate_id(&id),
                Authentication::PublicKey(pk) => rec.public_key == *pk,
            })
            .cloned()
            .next();
        rec_opt.ok_or_else(|| format_err!("Vault does not contain {:?}", auth))
    }

    fn signer_by_auth(&self, auth: &Authentication) -> Fallible<Box<dyn Signer>> {
        let bip32_idx = self.record_by_auth(auth)?.bip32_idx;
        let secret_key = self.private_key(bip32_idx)?;
        let signer = PrivateKeySigner::new(secret_key, auth.to_owned());
        Ok(Box::new(signer))
    }

    async fn create(&mut self, label_opt: Option<Label>) -> Fallible<DidVaultRecord> {
        let rec_idx = self.next_idx;
        let rec_idx_i32 = rec_idx as i32;
        let label = label_opt.unwrap_or(self.records.len().to_string());
        ensure!(self.did_by_label(&label).is_err(), "Label {} already exists in the vault", label);
        ensure!(self.records.len() == rec_idx, "Implementation error: index is not continuous");
        let key = self.public_key(rec_idx_i32)?;

        let rec = DidVaultRecord::new(rec_idx_i32, key.clone(), label);
        self.records.push(rec);

        self.active_idx = Option::Some(rec_idx);
        self.next_idx += 1;
        debug!("Active profile was set to {} at idx {}", key.key_id(), rec_idx,);

        Ok(self.records[rec_idx].to_owned())
    }

    async fn update(&mut self, record: DidVaultRecord) -> Fallible<()> {
        let idx = record.bip32_idx as usize;
        let old_rec = self
            .records
            .get(idx)
            .ok_or_else(|| format_err!("Index {} is invalid in record", idx))?;
        ensure!(old_rec.bip32_idx == record.bip32_idx, "Implementation error: invariant failed");
        let pub_key = self.public_key(record.bip32_idx)?;
        ensure!(old_rec.public_key == pub_key, "Public key is immutable in record");

        self.records[idx] = record;
        Ok(())
    }
}

pub struct PersistentDidVault {
    in_memory_vault: InMemoryDidVault,
    path: PathBuf,
}

impl PersistentDidVault {
    pub fn new(in_memory_vault: InMemoryDidVault, path: impl AsRef<Path>) -> Self {
        Self { in_memory_vault, path: path.as_ref().to_owned() }
    }

    pub async fn load(path: impl AsRef<Path>) -> Fallible<Self> {
        let path: &Path = path.as_ref();
        debug!("Loading DidVault from {:?}", path);
        let vault_file = File::open(path)?;
        let vault: InMemoryDidVault = serde_json::from_reader(&vault_file)?;
        //let vault: Self = bincode::deserialize_from(vault_file)?;
        if let Some(active) = vault.active_idx {
            ensure!(active < vault.next_idx, "active_idx cannot exceed last profile index");
        }
        ensure!(vault.next_idx as usize == vault.records.len(), "a record must exist for each id");

        use std::{collections::HashSet, iter::FromIterator};
        let unique_labels: HashSet<String> =
            HashSet::from_iter(vault.records.iter().map(|rec| rec.label.to_owned()));
        ensure!(vault.records.len() == unique_labels.len(), "all labels must be unique");

        Ok(Self::new(vault, path))
    }

    pub async fn save(&mut self) -> Fallible<()> {
        debug!("Saving profile vault to persist its state");
        if let Some(vault_dir) = self.path.parent() {
            debug!("Recursively Creating directory {:?}", vault_dir);
            std::fs::create_dir_all(vault_dir)?;
        }

        let vault_file = File::create(&self.path)?;
        serde_json::to_writer_pretty(&vault_file, &self.in_memory_vault)?;
        //bincode::serialize_into(vault_file, self)?;
        Ok(())
    }
}

#[async_trait(?Send)]
impl DidVault for PersistentDidVault {
    fn dids(&self) -> Fallible<Vec<Did>> {
        self.in_memory_vault.dids()
    }

    fn get_active(&self) -> Fallible<Option<Did>> {
        self.in_memory_vault.get_active()
    }

    async fn set_active(&mut self, did: &Did) -> Fallible<()> {
        self.in_memory_vault.set_active(did).await?;
        self.save().await
    }

    fn record_by_auth(&self, auth: &Authentication) -> Fallible<DidVaultRecord> {
        self.in_memory_vault.record_by_auth(auth)
    }

    fn signer_by_auth(&self, auth: &Authentication) -> Fallible<Box<dyn Signer>> {
        self.in_memory_vault.signer_by_auth(auth)
    }

    async fn create(&mut self, label_opt: Option<Label>) -> Fallible<DidVaultRecord> {
        let result = self.in_memory_vault.create(label_opt).await?;
        self.save().await?;
        Ok(result)
    }

    async fn update(&mut self, record: DidVaultRecord) -> Fallible<()> {
        self.in_memory_vault.update(record).await?;
        self.save().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEMO_PHRASE: &str = "include pear escape sail spy orange cute despair witness trouble sleep torch wire burst unable brass expose fiction drift clock duck oxygen aerobic already";

    fn in_memory_vault_instance() -> Fallible<InMemoryDidVault> {
        // TODO take this from some constant somewhere else
        let seed = Seed::from_bip39(DEMO_PHRASE)?;
        Ok(InMemoryDidVault::new(seed))
    }

    async fn test_vault<T: DidVault>(vault: &mut T) -> Fallible<()> {
        assert_eq!(vault.dids()?, vec![]);

        let record1 = vault.create(None).await?;
        assert_eq!(vault.dids()?, vec![record1.did()]);
        assert_eq!(record1.bip32_idx, 0);
        assert_eq!(record1.did().to_string(), "did:morpheus:ezbeWGSY2dqcUBqT8K7R14xr");

        let record2 = vault.create(None).await?;
        assert_eq!(vault.dids()?, vec![record1.did(), record2.did()]);
        assert_eq!(record2.bip32_idx, 1);
        assert_eq!(record2.did().to_string(), "did:morpheus:ez25N5WZ1Q6TQpgpyYgiu9gTX");

        Ok(())
    }

    #[tokio::test]
    async fn in_memory_vault() -> Fallible<()> {
        test_vault(&mut in_memory_vault_instance()?).await
    }

    #[tokio::test]
    async fn persistent_vault() -> Fallible<()> {
        let tmp_dir = tempfile::tempdir()?.into_path();
        let tmp_file = tmp_dir.join("morpheus-testvault.dat");
        //let tmp_file_str = tmp_file.into_os_string().into_string()?;
        let mem_vault = in_memory_vault_instance()?;
        let mut persistent_vault = PersistentDidVault::new(mem_vault, &tmp_file);
        test_vault(&mut persistent_vault).await?;

        let mem_vault = &persistent_vault.in_memory_vault;
        let loaded_vault = PersistentDidVault::load(&tmp_file).await?;
        let loaded_mem_vault = &loaded_vault.in_memory_vault;
        assert_eq!(loaded_mem_vault.active_idx, mem_vault.active_idx);
        assert_eq!(loaded_mem_vault.next_idx, mem_vault.next_idx);
        assert_eq!(loaded_mem_vault.records, mem_vault.records);
        Ok(())
    }
}
