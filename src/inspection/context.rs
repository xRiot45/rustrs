use crate::compatibility::{CompatibilityStatus, compatibility_status};
use crate::manifest::discovery::discover_manifest;
use crate::manifest::model::ProjectManifest;

use crate::template::metadata::TemplateMetadata;
use crate::template::resolver::resolve_template;
use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Finding {
    pub code: String,
    pub severity: Severity,
    pub title: String,
    pub detail: Option<String>,
    pub path: Option<PathBuf>,
    pub suggestion: Option<String>,
    pub certain: bool,
}

#[derive(Debug, Clone)]
pub struct InspectionContext {
    pub project_root: PathBuf,
    pub manifest_path: Option<PathBuf>,
    pub manifest: Option<ProjectManifest>,
    pub cargo_manifest_path: Option<PathBuf>,
    pub cargo_manifest: Option<toml::Value>,
    pub template_metadata: Option<&'static TemplateMetadata>,
    pub required_files: Vec<String>,
    pub compatibility: Option<CompatibilityStatus>,
    pub findings: Vec<Finding>,
}

impl InspectionContext {
    pub fn build(start: &Path) -> Result<Self, crate::error::CliError> {
        let mut findings = Vec::new();

        // Resolve project root via manifest first, then Cargo.toml.
        let discovered = discover_manifest(start).ok();
        let project_root = if let Some(ref d) = discovered {
            d.project_root.clone()
        } else {
            // Fallback to Cargo.toml discovery.
            let mut current = Some(start);
            let mut found = None;
            while let Some(dir) = current {
                if dir.join("Cargo.toml").is_file() {
                    found = Some(dir.to_path_buf());
                    break;
                }
                current = dir.parent();
            }
            found.unwrap_or_else(|| start.to_path_buf())
        };

        let project_root = std::fs::canonicalize(&project_root).map_err(|error| {
            crate::error::CliError::InvalidInput(format!("invalid project path: {error}"))
        })?;

        let manifest_path = discovered.as_ref().map(|d| d.manifest_path.clone());
        let manifest = discovered.map(|d| d.manifest);

        // Validate manifest if present.
        if let Some(m) = &manifest {
            if let Err(err) = m.validate() {
                findings.push(Finding {
                    code: "MANIFEST_INVALID".to_string(),
                    severity: Severity::Error,
                    title: "Project manifest is invalid".to_string(),
                    detail: Some(err.to_string()),
                    path: manifest_path.clone(),
                    suggestion: Some(
                        "Fix .rustrs/config.toml and rerun rustrs doctor.".to_string(),
                    ),
                    certain: true,
                });
            } else if let Ok(status) = resolve_template(m.template, m) {
                let _ = status;
            } else {
                // compatibility checked later
            }
        } else {
            findings.push(Finding {
                code: "MANIFEST_MISSING".to_string(),
                severity: Severity::Error,
                title: "Project manifest is missing".to_string(),
                detail: Some(".rustrs/config.toml was not found.".to_string()),
                path: Some(project_root.join(".rustrs").join("config.toml")),
                suggestion: Some("Run rustrs new or create the manifest.".to_string()),
                certain: true,
            });
        }

        // Compatibility.
        let compatibility = manifest.as_ref().map(compatibility_status);

        // Template metadata + required files.
        let mut template_metadata = None;
        let mut required_files = Vec::new();
        if let Some(m) = &manifest {
            if let Ok(resolved) = resolve_template(m.template, m) {
                template_metadata = Some(resolved.metadata);
                required_files = resolved
                    .metadata
                    .assets
                    .iter()
                    .filter(|a| a.required)
                    .map(|a| a.relative_path.to_string())
                    .collect();
            }
        }

        // Cargo manifest.
        let cargo_manifest_path = {
            let p = project_root.join("Cargo.toml");
            if p.is_file() { Some(p) } else { None }
        };
        let cargo_manifest = cargo_manifest_path
            .as_ref()
            .and_then(|p| std::fs::read_to_string(p).ok())
            .and_then(|s| toml::from_str::<toml::Value>(&s).ok());

        if cargo_manifest_path.is_none() {
            findings.push(Finding {
                code: "CARGO_MISSING".to_string(),
                severity: Severity::Error,
                title: "Cargo.toml is missing".to_string(),
                detail: None,
                path: Some(project_root.join("Cargo.toml")),
                suggestion: Some("Ensure the project root is correct.".to_string()),
                certain: true,
            });
        } else if cargo_manifest.is_none() {
            findings.push(Finding {
                code: "CARGO_INVALID".to_string(),
                severity: Severity::Error,
                title: "Cargo.toml is invalid".to_string(),
                detail: Some("Failed to parse Cargo.toml.".to_string()),
                path: cargo_manifest_path.clone(),
                suggestion: Some("Fix Cargo.toml syntax.".to_string()),
                certain: true,
            });
        }

        Ok(Self {
            project_root,
            manifest_path,
            manifest,
            cargo_manifest_path,
            cargo_manifest,
            template_metadata,
            required_files,
            compatibility,
            findings,
        })
    }
}
