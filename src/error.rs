use crate::project::root::ProjectRootError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("invalid input: {0}")]
    #[allow(dead_code)]
    InvalidInput(String),

    #[error(transparent)]
    ProjectRoot(#[from] ProjectRootError),
}

impl CliError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::InvalidInput(_) => 2,
            Self::ProjectRoot(_) => 2,
        }
    }
}
