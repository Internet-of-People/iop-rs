use serde::{Deserialize, Serialize};

use crate::crypto::{
    hash::{Content, ContentId},
    sign::{Nonce, Signable},
};
use crate::data::{did::Did, process::ProcessId, schema::MorpheusValue, serde_string};

pub type ClaimId = ContentId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Claim {
    #[serde(with = "serde_string")]
    subject: Did,
    content: MorpheusValue,
}

impl Content for Claim {}
impl Signable for Claim {}

// TODO Eq, PartialEq and maybe PartialOrd for WitnessRequest
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WitnessRequest {
    #[serde(with = "serde_string", rename = "processId")]
    process_id: ProcessId,
    claimant: String, // TODO should be an AuthenticationLink on the long term
    claim: Claim,
    evidence: MorpheusValue,
    nonce: Nonce, // TODO nonce was also added to Signed<T>, remove this later
}

impl Content for WitnessRequest {}
impl Signable for WitnessRequest {}

// TODO Eq, PartialEq and maybe PartialOrd for WitnessStatement
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WitnessStatement {
    #[serde(with = "serde_string", rename = "publicKey")]
    process_id: ProcessId,
    claim: Claim,
    witness: Did,
    constraints: MorpheusValue,
    nonce: Nonce, // TODO nonce was also added to Signed<T>, remove this later
}

impl Content for WitnessStatement {}
impl Signable for WitnessStatement {}
