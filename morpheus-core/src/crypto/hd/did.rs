use super::*;

pub trait SyncDidVault {
    fn key_ids(&self) -> Fallible<Vec<MKeyId>>;
    fn dids(&self) -> Fallible<Vec<Did>>;

    fn get_active(&self) -> Fallible<Option<Did>>;
    fn set_active(&mut self, did: &Did) -> Fallible<()>;

    fn record_by_auth(&self, auth: &Authentication) -> Fallible<HdRecord>;
    // fn restore_id(&mut self, did: &Did) -> Fallible<()>;
    // fn signer_by_auth(&self, auth: &Authentication) -> Fallible<Box<dyn SyncSigner>>;
    fn signer_by_auth(&self, auth: &Authentication) -> Fallible<PrivateKeySigner>;

    fn create(&mut self, label: Option<Label>) -> Fallible<HdRecord>;
    fn update(&mut self, record: HdRecord) -> Fallible<()>;
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
    records: Vec<HdRecord>,
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

    fn public_key(&self, idx: i32) -> Fallible<MPublicKey> {
        let did_xsk = self.morpheus_xsk()?.derive_hardened_child(idx)?;
        let key = did_xsk.neuter().public_key();
        Ok(key.into())
    }

    fn private_key(&self, idx: i32) -> Fallible<MPrivateKey> {
        let did_xsk = self.morpheus_xsk()?.derive_hardened_child(idx)?;
        let key = did_xsk.private_key();
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

    pub fn verify_integrity(&self) -> Fallible<()> {
        if let Some(active) = self.active_idx {
            ensure!(active < self.next_idx, "active_idx cannot exceed last profile index");
        }
        ensure!(self.next_idx as usize == self.records.len(), "a record must exist for each id");

        use std::{collections::HashSet, iter::FromIterator};
        let unique_labels: HashSet<String> =
            HashSet::from_iter(self.records.iter().map(|rec| rec.label.to_owned()));
        ensure!(self.records.len() == unique_labels.len(), "all labels must be unique");
        Ok(())
    }
}

impl SyncDidVault for InMemoryDidVault {
    fn key_ids(&self) -> Fallible<Vec<MKeyId>> {
        Ok(self.records.iter().map(|rec| rec.key_id()).collect())
    }

    fn dids(&self) -> Fallible<Vec<Did>> {
        Ok(self.records.iter().map(|rec| rec.did()).collect())
    }

    fn get_active(&self) -> Fallible<Option<Did>> {
        if let Some(idx) = self.active_idx {
            ensure!(idx < self.records.len(), "Implementation error: invalid active_idx");
        }
        Ok(self.active_idx.map(|idx| self.records[idx].did()))
    }

    fn set_active(&mut self, did: &Did) -> Fallible<()> {
        let idx = self
            .index_of_did(did)
            .ok_or_else(|| format_err!("Vault does not contain DID {}", did))?;
        self.active_idx = Some(idx);
        Ok(())
    }

    fn record_by_auth(&self, auth: &Authentication) -> Fallible<HdRecord> {
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

    fn signer_by_auth(&self, auth: &Authentication) -> Fallible<PrivateKeySigner> {
        let bip32_idx = self.record_by_auth(auth)?.bip32_idx;
        let secret_key = self.private_key(bip32_idx)?;
        let signer = PrivateKeySigner::new(secret_key);
        Ok(signer)
    }

    fn create(&mut self, label_opt: Option<Label>) -> Fallible<HdRecord> {
        let rec_idx = self.next_idx;
        let rec_idx_i32 = rec_idx as i32;
        let label = label_opt.unwrap_or_else(|| self.records.len().to_string());
        ensure!(self.did_by_label(&label).is_err(), "Label {} already exists in the vault", label);
        ensure!(self.records.len() == rec_idx, "Implementation error: index is not continuous");
        let key = self.public_key(rec_idx_i32)?;

        let rec = HdRecord::new(rec_idx_i32, key.clone(), label);
        self.records.push(rec);

        self.active_idx = Option::Some(rec_idx);
        self.next_idx += 1;
        debug!("Active profile was set to {} at idx {}", key.key_id(), rec_idx,);

        Ok(self.records[rec_idx].to_owned())
    }

    fn update(&mut self, record: HdRecord) -> Fallible<()> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use iop_keyvault::{Bip39, Seed};

    fn in_memory_vault_instance() -> Fallible<InMemoryDidVault> {
        let seed = Bip39::new().phrase(Seed::DEMO_PHRASE)?.password(Seed::PASSWORD);
        Ok(InMemoryDidVault::new(seed))
    }

    fn test_vault<T: SyncDidVault>(vault: &mut T) -> Fallible<()> {
        assert_eq!(vault.dids()?, vec![]);

        let record1 = vault.create(None)?;
        assert_eq!(vault.dids()?, vec![record1.did()]);
        assert_eq!(record1.bip32_idx, 0);
        assert_eq!(record1.did().to_string(), "did:morpheus:ezbeWGSY2dqcUBqT8K7R14xr");

        let record2 = vault.create(None)?;
        assert_eq!(vault.dids()?, vec![record1.did(), record2.did()]);
        assert_eq!(record2.bip32_idx, 1);
        assert_eq!(record2.did().to_string(), "did:morpheus:ez25N5WZ1Q6TQpgpyYgiu9gTX");

        Ok(())
    }

    #[test]
    fn in_memory_vault() -> Fallible<()> {
        test_vault(&mut in_memory_vault_instance()?)
    }
}
