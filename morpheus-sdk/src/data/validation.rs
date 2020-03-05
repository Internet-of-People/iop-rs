use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialOrd, PartialEq, Serialize)]
pub enum ValidationStatus {
    /// All possible checks are done and passed.
    Valid,
    /// Some checks could not be performed for lack of information, all others passed.
    /// E.g. Signatures are valid, but no timestamps are present so
    /// they could have been created outside the time period in which the signer key was valid.
    MaybeValid,
    /// Any step of validation failed.
    Invalid,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialOrd, PartialEq, Serialize)]
pub enum ValidationIssueSeverity {
    Warning,
    Error,
}

const VALIDATION_CODE_DEFAULT: u32 = 0;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialOrd, PartialEq, Serialize)]
pub struct ValidationIssue {
    code: u32,
    reason: String,
    severity: ValidationIssueSeverity,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialOrd, PartialEq, Serialize)]
pub struct ValidationResult {
    issues: Vec<ValidationIssue>,
}

impl ValidationResult {
    pub fn status(&self) -> ValidationStatus {
        let has_error = self.issues.iter().any(|it| it.severity == ValidationIssueSeverity::Error);
        if has_error {
            ValidationStatus::Invalid
        } else if !self.issues.is_empty() {
            ValidationStatus::MaybeValid
        } else {
            ValidationStatus::Valid
        }
    }

    pub fn add_issue(&mut self, severity: ValidationIssueSeverity, reason: &str) {
        self.issues.push(ValidationIssue {
            severity,
            code: VALIDATION_CODE_DEFAULT,
            reason: reason.to_owned(),
        })
    }

    pub fn issues(&self) -> &[ValidationIssue] {
        self.issues.as_slice()
    }
}
