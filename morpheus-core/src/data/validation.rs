use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialOrd, PartialEq, Serialize)]
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

impl std::fmt::Display for ValidationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Valid => "valid",
            Self::MaybeValid => "maybe valid",
            Self::Invalid => "invalid",
        };
        write!(f, "{}", msg)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialOrd, PartialEq, Serialize)]
pub enum ValidationIssueSeverity {
    Warning,
    Error,
}

impl std::fmt::Display for ValidationIssueSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::Warning => "warning",
            Self::Error => "error",
        };
        write!(f, "{}", msg)
    }
}

const VALIDATION_CODE_DEFAULT: u32 = 0;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialOrd, PartialEq, Serialize)]
pub struct ValidationIssue {
    code: u32,
    reason: String,
    severity: ValidationIssueSeverity,
}

impl ValidationIssue {
    pub fn code(&self) -> u32 {
        self.code
    }
    pub fn reason(&self) -> &str {
        &self.reason
    }
    pub fn severity(&self) -> ValidationIssueSeverity {
        self.severity
    }
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
