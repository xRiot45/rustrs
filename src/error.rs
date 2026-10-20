use crate::compatibility::CompatibilityError;
use crate::manifest::ManifestError;
use crate::project::root::ProjectRootError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("invalid input: {0}")]
    #[allow(dead_code)]
    InvalidInput(String),

    #[error(transparent)]
    ProjectRoot(#[from] ProjectRootError),

    #[error(transparent)]
    Manifest(#[from] ManifestError),

    #[error(transparent)]
    Compatibility(#[from] CompatibilityError),
}

impl CliError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::InvalidInput(_) => 2,
            Self::ProjectRoot(_) => 2,
            Self::Manifest(error) => match error {
                ManifestError::Io { .. } => 1,
                _ => 2,
            },
            Self::Compatibility(_) => 2,
        }
    }
}
