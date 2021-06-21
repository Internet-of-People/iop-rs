use super::*;

use crate::txtype::OperationAttempt;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationError {
    pub invalid_operation_attempt: Option<OperationAttempt>,
    // code: u16; TODO: later we need exact error codes
    pub message: String,
}
