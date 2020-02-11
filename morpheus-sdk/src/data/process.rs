use serde::{Deserialize, Serialize};

use crate::crypto::{hash::Content, sign::Signable};
use crate::data::schema::MorpheusSchema;

pub type ProcessId = String; // TODO use something like a ContentId here

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub struct Process {
    // TODO do we need a separate 'id' field here?
    name: String,
    version: u32,
    description: String,
    evidence_schema: MorpheusSchema,
    constraints_schema: MorpheusSchema,
    claim_schema: MorpheusSchema,
}

impl Content for Process {}
impl Signable for Process {}
