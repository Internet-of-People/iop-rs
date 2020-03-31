use async_trait::async_trait;
use failure::Fallible;

use iop_morpheus_core::data::process::{Process, ProcessId};

// TODO consider if this worths a separate interface or should be merged into e.g. ClaimManager
// TODO how to add a new process? Is it done within this same interface or via a separate one?
#[async_trait(?Send)]
pub trait ContentRepository {
    async fn get_process(&self, id: &ProcessId) -> Fallible<Process>;
}
