use crate::compatibility::matrix;
use crate::compatibility::types::{CompatibilityError, CompatibilityStatus};
use crate::manifest::model::ProjectManifest;

pub fn compatibility_status(manifest: &ProjectManifest) -> CompatibilityStatus {
    matrix::evaluate(manifest.framework, manifest.database, manifest.data_access)
}

pub fn check_compatibility(manifest: &ProjectManifest) -> Result<(), CompatibilityError> {
    match compatibility_status(manifest) {
        CompatibilityStatus::Supported => Ok(()),
        CompatibilityStatus::Unsupported {
            reason,
            alternatives,
        } => Err(CompatibilityError::Unsupported {
            reason,
            alternatives,
        }),
    }
}
