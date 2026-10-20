#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompatibilityStatus {
    Supported,
    Unsupported {
        reason: String,
        alternatives: Vec<String>,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum CompatibilityError {
    #[error("unsupported combination: {reason}")]
    Unsupported {
        reason: String,
        alternatives: Vec<String>,
    },
}
