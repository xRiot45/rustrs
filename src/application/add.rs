use crate::cli::{AddArgs, Output};
use crate::compatibility::check_compatibility;
use crate::error::CliError;
use crate::manifest::discovery::discover_manifest;
use crate::manifest::model::Capability;
use std::env;

pub fn execute(args: &AddArgs, output: &Output) -> Result<(), CliError> {
    let start = args
        .path
        .clone()
        .unwrap_or_else(|| env::current_dir().unwrap_or_default());

    let discovered = discover_manifest(&start)?;
    discovered.manifest.validate()?;
    check_compatibility(&discovered.manifest)?;

    if !args.capability.is_implemented() {
        return Err(CliError::InvalidInput(format!(
            "capability is not implemented in the current MVP: {} ({})",
            args.capability,
            args.capability.effect_summary()
        )));
    }

    let root = discovered.project_root.clone();
    let mut manifest = discovered.manifest.clone();

    if manifest.features.is_enabled(args.capability) && !args.force && !output.quiet {
        output.info(&format!(
            "Capability {} is already enabled.",
            args.capability
        ));
    }

    match args.capability {
        Capability::Docker => {
            crate::application::assets::generate_docker_assets(
                &root,
                &mut manifest,
                args.force,
                args.dry_run,
                output,
            )?;
        }
        Capability::Cicd => {
            crate::application::assets::generate_ci_assets(
                &root,
                &mut manifest,
                args.provider,
                args.force,
                args.dry_run,
                output,
            )?;
        }
        Capability::Auth => {
            crate::application::assets::generate_auth_assets(
                &root,
                &mut manifest,
                false,
                args.force,
                args.dry_run,
                output,
            )?;
        }
        Capability::Rbac => {
            crate::application::assets::generate_auth_assets(
                &root,
                &mut manifest,
                true,
                args.force,
                args.dry_run,
                output,
            )?;
        }
        Capability::OpenApi => {
            crate::application::assets::generate_openapi_assets(
                &root,
                &mut manifest,
                args.force,
                args.dry_run,
                output,
            )?;
        }
        Capability::Redis => {
            crate::application::assets::generate_redis_assets(
                &root,
                &mut manifest,
                args.force,
                args.dry_run,
                output,
            )?;
        }
        Capability::Email => {
            crate::application::assets::generate_simple_capability_assets(
                &root,
                &mut manifest,
                Capability::Email,
                "email",
                crate::template::metadata::EMAIL_MODULE_RS,
                args.force,
                args.dry_run,
                output,
            )?;
        }
        Capability::Storage => {
            crate::application::assets::generate_simple_capability_assets(
                &root,
                &mut manifest,
                Capability::Storage,
                "storage",
                crate::template::metadata::STORAGE_MODULE_RS,
                args.force,
                args.dry_run,
                output,
            )?;
        }
        Capability::AuditLog => {
            crate::application::assets::generate_simple_capability_assets(
                &root,
                &mut manifest,
                Capability::AuditLog,
                "audit_log",
                crate::template::metadata::AUDIT_LOG_MODULE_RS,
                args.force,
                args.dry_run,
                output,
            )?;
        }
        Capability::RateLimit => {
            crate::application::assets::generate_simple_capability_assets(
                &root,
                &mut manifest,
                Capability::RateLimit,
                "rate_limit",
                crate::template::metadata::RATE_LIMIT_MODULE_RS,
                args.force,
                args.dry_run,
                output,
            )?;
        }
        Capability::Idempotency => {
            crate::application::assets::generate_simple_capability_assets(
                &root,
                &mut manifest,
                Capability::Idempotency,
                "idempotency",
                crate::template::metadata::IDEMPOTENCY_MODULE_RS,
                args.force,
                args.dry_run,
                output,
            )?;
        }
        Capability::Observability => {
            crate::application::assets::generate_simple_capability_assets(
                &root,
                &mut manifest,
                Capability::Observability,
                "observability",
                crate::template::metadata::OBSERVABILITY_MODULE_RS,
                args.force,
                args.dry_run,
                output,
            )?;
        }
    }

    if !output.quiet && !args.dry_run {
        output.info(&format!(
            "Capability {} added: {}.",
            args.capability,
            args.capability.effect_summary()
        ));
    }

    Ok(())
}
