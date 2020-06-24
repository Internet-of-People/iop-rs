use async_trait::async_trait;
use failure::Fallible;

use crate::io::local::signer::MorpheusSigner;
use iop_keyvault::multicipher::MKeyId;
use iop_morpheus_core::crypto::hd::{HdRecord, Label};

pub struct HydraRecord {}

#[async_trait(?Send)]
pub trait HydraWallet {
    fn addresses(&self) -> Fallible<Vec<MKeyId>>;

    fn get_active(&self) -> Fallible<Option<MKeyId>>;
    async fn set_active(&mut self, did: &MKeyId) -> Fallible<()>;

    fn record_by_address(&self, auth: &MKeyId) -> Fallible<HydraRecord>;
    // async fn restore_id(&mut self, did: &Did) -> Fallible<()>;
    fn signer_by_address(&self, auth: &MKeyId) -> Fallible<Box<dyn MorpheusSigner>>;

    async fn create(&mut self, label: Option<Label>) -> Fallible<HdRecord>;
    async fn update(&mut self, record: HdRecord) -> Fallible<()>;
}

// pub struct PersistentHydraVault {
//     in_memory_vault: InMemoryDidVault,
//     persister: Box<dyn Persister>,
// }
