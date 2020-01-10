use serde::{Deserialize, Serialize};

use crate::data::{
    did::Did,
    hash::{Content, ContentId},
    process::ProcessId,
    schema::MorpheusValue,
    sign::{Nonce, Signable},
};

pub type ClaimId = ContentId;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub struct Claim {
    subject: Did,
    content: MorpheusValue,
}

impl Content for Claim {}
impl Signable for Claim {}

// TODO Eq, PartialEq and maybe PartialOrd for WitnessRequest
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WitnessRequest {
    process: ProcessId,
    claim: Claim,
    evidence: MorpheusValue,
    nonce: Nonce, // TODO consider if nonces conceptually belong here or near Signed<T> instead
}

impl Content for WitnessRequest {}
impl Signable for WitnessRequest {}

// TODO Eq, PartialEq and maybe PartialOrd for WitnessStatement
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WitnessStatement {
    process: ProcessId,
    claim: Claim,
    witness: Did,
    constraints: MorpheusValue,
    nonce: Nonce, // TODO consider if nonces conceptually belong here or near Signed<T> instead
}

impl Content for WitnessStatement {}
impl Signable for WitnessStatement {}
