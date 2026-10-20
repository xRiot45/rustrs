use std::io;
use std::path::{Path, PathBuf};

use std::fmt;

#[derive(Debug)]
pub enum PathError {
    Absolute { path: PathBuf },
    ParentTraversal { path: PathBuf },
    InvalidComponent { path: PathBuf },
    Escape { path: PathBuf },
    Io { path: PathBuf, source: io::Error },
}

#[derive(Debug, thiserror::Error)]
pub enum TemplatePathError {
    #[error("invalid path: {path}")]
    Invalid { path: PathBuf },
}

pub fn validate_relative_path(path: &Path) -> Result<PathBuf, PathError> {
    if path.is_absolute() {
        return Err(PathError::Absolute {
            path: path.to_path_buf(),
        });
    }

    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::Normal(part) => {
                normalized.push(part);
            }
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                return Err(PathError::ParentTraversal {
                    path: path.to_path_buf(),
                });
            }
            std::path::Component::RootDir | std::path::Component::Prefix(_) => {
                return Err(PathError::Absolute {
                    path: path.to_path_buf(),
                });
            }
        }
    }

    if normalized.as_os_str().is_empty() {
        return Err(PathError::InvalidComponent {
            path: path.to_path_buf(),
        });
    }

    Ok(normalized)
}

impl fmt::Display for PathError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Absolute { path } => write!(
                formatter,
                "absolute path is not allowed: {}",
                path.display()
            ),
            Self::ParentTraversal { path } => {
                write!(
                    formatter,
                    "parent traversal is not allowed: {}",
                    path.display()
                )
            }
            Self::InvalidComponent { path } => {
                write!(formatter, "invalid path component: {}", path.display())
            }
            Self::Escape { path } => {
                write!(formatter, "path escapes project root: {}", path.display())
            }
            Self::Io { path, source } => {
                write!(
                    formatter,
                    "failed to inspect path {}: {}",
                    path.display(),
                    source
                )
            }
        }
    }
}

pub fn ensure_within_root(root: &Path, candidate: &Path) -> Result<PathBuf, PathError> {
    let validated = validate_relative_path(candidate)?;
    let joined = root.join(&validated);

    let canonical_root = std::fs::canonicalize(root).map_err(|source| PathError::Io {
        path: root.to_path_buf(),
        source,
    })?;

    if joined.exists() {
        let canonical_joined = std::fs::canonicalize(&joined).map_err(|source| PathError::Io {
            path: joined.clone(),
            source,
        })?;
        if !canonical_joined.starts_with(&canonical_root) {
            return Err(PathError::Escape {
                path: candidate.to_path_buf(),
            });
        }
        return Ok(canonical_joined);
    }

    // For non-existing targets, validate the nearest existing parent.
    let mut existing_parent = joined.as_path();
    while !existing_parent.exists() {
        match existing_parent.parent() {
            Some(parent) => existing_parent = parent,
            None => break,
        }
    }

    if existing_parent.exists() {
        let canonical_parent =
            std::fs::canonicalize(existing_parent).map_err(|source| PathError::Io {
                path: existing_parent.to_path_buf(),
                source,
            })?;
        if !canonical_parent.starts_with(&canonical_root) {
            return Err(PathError::Escape {
                path: candidate.to_path_buf(),
            });
        }
    }

    Ok(joined)
}
