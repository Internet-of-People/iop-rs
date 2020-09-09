use super::*;

use crate::io::local::signer::MorpheusSigner;
use iop_keyvault::multicipher::MKeyId;
use iop_morpheus_core::crypto::hd::{HdRecord, Label};

pub struct HydraRecord {}

#[async_trait(?Send)]
pub trait HydraWallet {
    fn addresses(&self) -> Result<Vec<MKeyId>>;

    fn get_active(&self) -> Result<Option<MKeyId>>;
    async fn set_active(&mut self, did: &MKeyId) -> Result<()>;

    fn record_by_address(&self, auth: &MKeyId) -> Result<HydraRecord>;
    // async fn restore_id(&mut self, did: &Did) -> Result<()>;
    fn signer_by_address(&self, auth: &MKeyId) -> Result<Box<dyn MorpheusSigner>>;

    async fn create(&mut self, label: Option<Label>) -> Result<HdRecord>;
    async fn update(&mut self, record: HdRecord) -> Result<()>;
}

// pub struct PersistentHydraVault {
//     in_memory_vault: InMemoryDidVault,
//     persister: Box<dyn Persister>,
// }
