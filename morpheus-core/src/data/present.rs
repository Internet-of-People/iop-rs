use serde::{Deserialize, Serialize};

use crate::{
    crypto::{
        hash::Content,
        sign::{Nonce, Signable, Signed},
    },
    data::{did::Did, serde_string},
};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub struct License {
    #[serde(rename = "issuedTo", with = "serde_string")]
    issued_to: Did,
    purpose: String, // TODO should be more strictly typed, probably an enum
    #[serde(rename = "validFrom")]
    valid_from: String, // TODO should be some strict date type here, like std::time::Instant but it's not serde-serializable
    #[serde(rename = "validUntil")]
    valid_until: String,
}

impl Content for License {}
impl Signable for License {}

// TODO this probably should be more strictly typed here
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProvenClaim {
    claim: serde_json::Value,
    statements: Vec<Signed<serde_json::Value>>,
}

impl Content for ProvenClaim {}
impl Signable for ProvenClaim {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ClaimPresentation {
    #[serde(rename = "provenClaims")]
    proven_claims: Vec<ProvenClaim>,
    licenses: Vec<License>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    nonce: Option<Nonce>,
    // if subjects are different (from each other or the creator of this presentation)
    // then the creator an optional license is needed to prove proper rights to further delegate claims
    // consider how to do it without potentially infinite data size?
    // claim_control_proof: Option<Signed<ClaimPresentation>>,
}

impl Content for ClaimPresentation {}
impl Signable for ClaimPresentation {}

// TODO Maskable: T -> serde_json::Value
