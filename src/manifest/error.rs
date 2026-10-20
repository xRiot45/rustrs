use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CapabilityError {
    #[error("capability {capability} requires {requires}")]
    Dependency {
        capability: &'static str,
        requires: &'static str,
    },
}

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("invalid path: {path}")]
    InvalidPath { path: PathBuf },

    #[error("manifest not found starting from {start}")]
    NotFound { start: PathBuf },

    #[error("failed to read manifest {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("failed to parse manifest: {message}")]
    Parse { message: String },

    #[error("unsupported schema version: {version}")]
    UnsupportedSchemaVersion { version: u32 },

    #[error("invalid manifest: {message}")]
    Validation { message: String },

    #[error(transparent)]
    Capability(#[from] CapabilityError),
}
