use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WritePolicy {
    PreserveExisting,
    Overwrite,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedOperation {
    pub relative_path: PathBuf,
    pub contents: Vec<u8>,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerationPlan {
    pub target_root: PathBuf,
    pub operations: Vec<PlannedOperation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperationOutcome {
    Planned,
    Created,
    Skipped { reason: String },
    Failed { error: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationResult {
    pub relative_path: PathBuf,
    pub outcome: OperationOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationReport {
    pub target_root: PathBuf,
    pub results: Vec<OperationResult>,
}
