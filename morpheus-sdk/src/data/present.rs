use serde::{Deserialize, Serialize};

use crate::crypto::{hash::Content, sign::Signable};
use crate::data::{claim::Claim, did::Did};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub struct License {
    // TODO how to build a proper license?
    // license_type
    licensed_to: Did,
    // issued_at or valid_from
    // expires_at or valid_until
}

impl Content for License {}
impl Signable for License {}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub struct ClaimPresentation {
    // TODO how to represent Merkle-tree and other data here?
    // a collection of claims, potentially on different subjects
    claims: Vec<Claim>,
    // if subjects are different then the creator of this presentation then an optional license is needed to prove proper rights to further delegate claims
    // consider how to do it without potentially infinite data size?
    // claim_control_proof: Option<Signed<ClaimPresentation>>,
    license: License,
}

impl Content for ClaimPresentation {}
impl Signable for ClaimPresentation {}
