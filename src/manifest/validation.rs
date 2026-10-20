use crate::manifest::error::ManifestError;
use crate::manifest::model::ProjectManifest;

pub const SUPPORTED_SCHEMA_VERSION: u32 = 1;

pub fn validate_schema_version(version: u32) -> Result<(), ManifestError> {
    if version == SUPPORTED_SCHEMA_VERSION {
        Ok(())
    } else {
        Err(ManifestError::UnsupportedSchemaVersion { version })
    }
}

pub fn validate_project_name(name: &str) -> Result<(), ManifestError> {
    if name.is_empty() {
        return Err(ManifestError::Validation {
            message: "project_name must not be empty".to_string(),
        });
    }

    let mut chars = name.chars();
    let first = chars.next().unwrap();
    let last = name.chars().last().unwrap();

    if !first.is_ascii_lowercase() {
        return Err(ManifestError::Validation {
            message: "project_name must start with a lowercase ASCII letter".to_string(),
        });
    }
    if !last.is_ascii_alphanumeric() {
        return Err(ManifestError::Validation {
            message: "project_name must end with a lowercase ASCII letter or digit".to_string(),
        });
    }

    let mut previous_hyphen = false;
    for ch in name.chars() {
        let valid = ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-';
        if !valid {
            return Err(ManifestError::Validation {
                message:
                    "project_name may contain only lowercase ASCII letters, digits, and hyphens"
                        .to_string(),
            });
        }
        if ch == '-' {
            if previous_hyphen {
                return Err(ManifestError::Validation {
                    message: "project_name must not contain consecutive hyphens".to_string(),
                });
            }
            previous_hyphen = true;
        } else {
            previous_hyphen = false;
        }
    }

    Ok(())
}

impl ProjectManifest {
    pub fn validate(&self) -> Result<(), ManifestError> {
        validate_schema_version(self.schema_version)?;
        validate_project_name(&self.project_name)?;
        self.features.validate()?;
        Ok(())
    }
}
