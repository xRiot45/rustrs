use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum ProjectRootError {
    #[error("invalid path: {path}")]
    InvalidPath { path: PathBuf },

    #[error("project root not found starting from {start}")]
    NotFound { start: PathBuf },

    #[error("failed to inspect path {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

#[allow(dead_code)]
pub fn discover_project_root(start: &Path) -> Result<PathBuf, ProjectRootError> {
    let canonical_start = fs::canonicalize(start).map_err(|source| {
        if source.kind() == std::io::ErrorKind::NotFound {
            ProjectRootError::InvalidPath {
                path: start.to_path_buf(),
            }
        } else {
            ProjectRootError::Io {
                path: start.to_path_buf(),
                source,
            }
        }
    })?;

    if !canonical_start.is_dir() {
        return Err(ProjectRootError::InvalidPath {
            path: start.to_path_buf(),
        });
    }

    let mut current = canonical_start;
    loop {
        if current.join("Cargo.toml").is_file() {
            return Ok(current);
        }

        match current.parent() {
            Some(parent) if parent != current => current = parent.to_path_buf(),
            _ => break,
        }
    }

    Err(ProjectRootError::NotFound {
        start: start.to_path_buf(),
    })
}
