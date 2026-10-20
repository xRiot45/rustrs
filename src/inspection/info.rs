use crate::cli::Output;
use crate::error::CliError;
use crate::inspection::structure::check_required_files;
use crate::inspection::{InspectionContext, Severity};
use serde_json::json;

pub fn execute(ctx: &InspectionContext, output: &Output, json: bool) -> Result<(), CliError> {
    let manifest = ctx.manifest.as_ref().ok_or_else(|| {
        CliError::InvalidInput("project manifest is missing or invalid".to_string())
    })?;

    if json {
        let value = json!({
            "project_name": manifest.project_name,
            "project_root": ctx.project_root,
            "manifest_path": ctx.manifest_path,
            "schema_version": manifest.schema_version,
            "template": format!("{:?}", manifest.template).to_lowercase(),
            "framework": format!("{:?}", manifest.framework).to_lowercase(),
            "database": format!("{:?}", manifest.database).to_lowercase(),
            "data_access": format!("{:?}", manifest.data_access).to_lowercase(),
            "architecture": format!("{:?}", manifest.architecture).to_lowercase(),
            "features": manifest.features,
            "compatibility": format!("{:?}", ctx.compatibility),
            "warnings": ctx.findings,
        });
        output.info(&serde_json::to_string_pretty(&value).unwrap_or_default());
        return Ok(());
    }

    output.info(&format!("Project: {}", manifest.project_name));
    output.info(&format!("Root: {}", ctx.project_root.display()));
    if let Some(p) = &ctx.manifest_path {
        output.info(&format!("Manifest: {}", p.display()));
    }
    output.info(&format!("Schema: {}", manifest.schema_version));
    output.info(&format!("Template: {:?}", manifest.template));
    output.info(&format!("Framework: {:?}", manifest.framework));
    output.info(&format!("Database: {:?}", manifest.database));
    output.info(&format!("Data access: {:?}", manifest.data_access));
    output.info(&format!("Architecture: {:?}", manifest.architecture));
    output.info("Features:");
    output.info(&format!("  auth: {}", manifest.features.auth));
    output.info(&format!("  rbac: {}", manifest.features.rbac));
    output.info(&format!("  openapi: {}", manifest.features.openapi));
    output.info(&format!("  redis: {}", manifest.features.redis));
    output.info(&format!("  email: {}", manifest.features.email));
    output.info(&format!("  storage: {}", manifest.features.storage));
    output.info(&format!("  audit_log: {}", manifest.features.audit_log));
    output.info(&format!("  rate_limit: {}", manifest.features.rate_limit));
    output.info(&format!("  idempotency: {}", manifest.features.idempotency));
    output.info(&format!(
        "  observability: {}",
        manifest.features.observability
    ));
    output.info(&format!("  docker: {}", manifest.features.docker));
    output.info(&format!("  cicd: {}", manifest.features.cicd));
    if let Some(manifest) = &ctx.manifest {
        let validation = manifest.validate();
        output.info(&format!(
            "Manifest validation: {}",
            if validation.is_ok() {
                "valid"
            } else {
                "invalid"
            }
        ));
    }
    if let Some(status) = &ctx.compatibility {
        output.info(&format!("Compatibility: {:?}", status));
    }
    for f in check_required_files(ctx) {
        if f.severity != Severity::Info {
            output.warn(&format!("[{:?}] {}: {}", f.severity, f.code, f.title));
        }
    }
    for f in &ctx.findings {
        if f.severity != Severity::Info {
            output.warn(&format!("[{:?}] {}: {}", f.severity, f.code, f.title));
        }
    }
    Ok(())
}
