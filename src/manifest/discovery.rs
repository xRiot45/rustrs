use crate::manifest::error::ManifestError;
use crate::manifest::model::ProjectManifest;
use crate::manifest::parser::parse_manifest;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveredManifest {
    pub project_root: PathBuf,
    pub manifest_path: PathBuf,
    pub manifest: ProjectManifest,
}

pub fn discover_manifest(start: &Path) -> Result<DiscoveredManifest, ManifestError> {
    if !start.exists() {
        return Err(ManifestError::InvalidPath {
            path: start.to_path_buf(),
        });
    }
    if !start.is_dir() {
        return Err(ManifestError::InvalidPath {
            path: start.to_path_buf(),
        });
    }

    let mut current = Some(start);
    while let Some(dir) = current {
        let manifest_path = dir.join(".rustrs").join("config.toml");
        if manifest_path.is_file() {
            let contents =
                fs::read_to_string(&manifest_path).map_err(|source| ManifestError::Io {
                    path: manifest_path.clone(),
                    source,
                })?;
            let manifest = parse_manifest(&contents).map_err(|error| match error {
                ManifestError::Parse { message } => ManifestError::Parse {
                    message: format!("{}: {}", manifest_path.display(), message),
                },
                other => other,
            })?;
            return Ok(DiscoveredManifest {
                project_root: dir.to_path_buf(),
                manifest_path,
                manifest,
            });
        }
        current = dir.parent();
    }

    Err(ManifestError::NotFound {
        start: start.to_path_buf(),
    })
}
