use crate::manifest::error::CapabilityError;
use crate::manifest::model::{DataAccess, Database, Features, ProjectManifest};

pub fn validate_capabilities(features: &Features) -> Result<(), CapabilityError> {
    if features.rbac && !features.auth {
        return Err(CapabilityError::Dependency {
            capability: "rbac",
            requires: "auth",
        });
    }
    Ok(())
}

pub fn validate_project_capabilities(manifest: &ProjectManifest) -> Result<(), CapabilityError> {
    validate_capabilities(&manifest.features)?;
    if manifest.features.auth
        && (manifest.database != Database::Postgres || manifest.data_access != DataAccess::Sqlx)
    {
        return Err(CapabilityError::Dependency {
            capability: "auth",
            requires: "axum + postgres + sqlx",
        });
    }
    if manifest.features.audit_log
        && (manifest.database != Database::Postgres || manifest.data_access != DataAccess::Sqlx)
    {
        return Err(CapabilityError::Dependency {
            capability: "audit_log",
            requires: "axum + postgres + sqlx",
        });
    }
    Ok(())
}
