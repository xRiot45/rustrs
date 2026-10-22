use crate::cli::{
    ConfigArgs, ConfigCommands, ConfigGetArgs, ConfigSetArgs, ConfigShowArgs, Output,
};
use crate::compatibility::check_compatibility;
use crate::error::CliError;
use crate::filesystem::apply_plan;
use crate::generation::plan::{GenerationPlan, PlannedOperation, WritePolicy};
use crate::manifest::discovery::discover_manifest;
use crate::manifest::model::ProjectManifest;
use std::env;
use std::path::PathBuf;

pub fn execute(args: &ConfigArgs, output: &Output) -> Result<(), CliError> {
    let start = args
        .path
        .clone()
        .unwrap_or_else(|| env::current_dir().unwrap_or_default());

    match &args.command {
        ConfigCommands::Show(show) => show_config(&start, show, output),
        ConfigCommands::Get(get) => get_config(&start, get, output),
        ConfigCommands::Set(set) => set_config(&start, set, output),
    }
}

fn show_config(
    start: &std::path::Path,
    args: &ConfigShowArgs,
    output: &Output,
) -> Result<(), CliError> {
    let discovered = discover_manifest(start)?;
    discovered.manifest.validate()?;

    if args.json {
        let json = serde_json::to_string_pretty(&discovered.manifest)
            .map_err(|error| CliError::InvalidInput(error.to_string()))?;
        output.info(&json);
    } else {
        let toml = toml::to_string_pretty(&discovered.manifest)
            .map_err(|error| CliError::InvalidInput(error.to_string()))?;
        output.info(&toml);
    }

    Ok(())
}

fn get_config(
    start: &std::path::Path,
    args: &ConfigGetArgs,
    output: &Output,
) -> Result<(), CliError> {
    let discovered = discover_manifest(start)?;
    discovered.manifest.validate()?;

    let value = get_value(&discovered.manifest, &args.key)?;
    if args.json {
        let json = serde_json::to_string(&value)
            .map_err(|error| CliError::InvalidInput(error.to_string()))?;
        output.info(&json);
    } else {
        output.info(&value);
    }

    Ok(())
}

fn set_config(
    start: &std::path::Path,
    args: &ConfigSetArgs,
    output: &Output,
) -> Result<(), CliError> {
    let discovered = discover_manifest(start)?;
    let mut manifest = discovered.manifest.clone();

    set_value(&mut manifest, &args.key, &args.value)?;
    manifest.validate()?;
    check_compatibility(&manifest)?;

    let serialized = toml::to_string_pretty(&manifest)
        .map_err(|error| CliError::InvalidInput(error.to_string()))?;

    let plan = GenerationPlan {
        target_root: discovered.project_root.clone(),
        operations: vec![PlannedOperation {
            relative_path: PathBuf::from(".rustrs").join("config.toml"),
            contents: serialized.into_bytes(),
            description: "update .rustrs/config.toml".to_string(),
            overwrite: true,
        }],
    };

    let report = apply_plan(&plan, WritePolicy::Overwrite);
    for result in &report.results {
        let line = match &result.outcome {
            crate::generation::plan::OperationOutcome::Planned => {
                format!("  PLAN   {}", result.relative_path.display())
            }
            crate::generation::plan::OperationOutcome::Created => {
                format!("  CREATE {}", result.relative_path.display())
            }
            crate::generation::plan::OperationOutcome::Skipped { reason } => {
                format!("  SKIP   {} ({})", result.relative_path.display(), reason)
            }
            crate::generation::plan::OperationOutcome::Failed { error } => {
                format!("  FAIL   {} ({})", result.relative_path.display(), error)
            }
        };
        if !output.quiet {
            output.info(&line);
        }
    }

    let mut failed = false;
    for result in &report.results {
        if matches!(
            result.outcome,
            crate::generation::plan::OperationOutcome::Failed { .. }
        ) {
            failed = true;
        }
    }
    if failed {
        return Err(CliError::InvalidInput(
            "config update failed for one or more files".to_string(),
        ));
    }

    if !output.quiet {
        output.info(&format!(
            "Updated .rustrs/config.toml: {} = {}",
            args.key, args.value
        ));
        output.info("Manifest updated; generated project files are not regenerated automatically.");
    }

    Ok(())
}

fn get_value(manifest: &ProjectManifest, key: &str) -> Result<String, CliError> {
    match key {
        "schema_version" => Ok(manifest.schema_version.to_string()),
        "project_name" => Ok(manifest.project_name.clone()),
        "template" => Ok(serde_variant_name(&manifest.template)),
        "framework" => Ok(serde_variant_name(&manifest.framework)),
        "architecture" => Ok(serde_variant_name(&manifest.architecture)),
        "database" => Ok(serde_variant_name(&manifest.database)),
        "data_access" => Ok(serde_variant_name(&manifest.data_access)),
        "features.auth" => Ok(manifest.features.auth.to_string()),
        "features.rbac" => Ok(manifest.features.rbac.to_string()),
        "features.openapi" => Ok(manifest.features.openapi.to_string()),
        "features.redis" => Ok(manifest.features.redis.to_string()),
        "features.email" => Ok(manifest.features.email.to_string()),
        "features.storage" => Ok(manifest.features.storage.to_string()),
        "features.audit_log" => Ok(manifest.features.audit_log.to_string()),
        "features.rate_limit" => Ok(manifest.features.rate_limit.to_string()),
        "features.idempotency" => Ok(manifest.features.idempotency.to_string()),
        "features.observability" => Ok(manifest.features.observability.to_string()),
        "features.docker" => Ok(manifest.features.docker.to_string()),
        "features.cicd" => Ok(manifest.features.cicd.to_string()),
        _ => Err(CliError::InvalidInput(format!("invalid config key: {key}"))),
    }
}

fn set_value(manifest: &mut ProjectManifest, key: &str, value: &str) -> Result<(), CliError> {
    match key {
        "schema_version" => Err(CliError::InvalidInput(
            "schema_version is read-only".to_string(),
        )),
        "project_name" => {
            manifest.project_name = value.to_string();
            Ok(())
        }
        "template" => {
            manifest.template = parse_enum(value, "template")?;
            Ok(())
        }
        "framework" => {
            manifest.framework = parse_enum(value, "framework")?;
            Ok(())
        }
        "architecture" => {
            manifest.architecture = parse_enum(value, "architecture")?;
            Ok(())
        }
        "database" => {
            manifest.database = parse_enum(value, "database")?;
            Ok(())
        }
        "data_access" => {
            manifest.data_access = parse_enum(value, "data_access")?;
            Ok(())
        }
        "features.auth" => {
            manifest.features.auth = parse_bool(value, key)?;
            Ok(())
        }
        "features.rbac" => {
            manifest.features.rbac = parse_bool(value, key)?;
            Ok(())
        }
        "features.openapi" => {
            manifest.features.openapi = parse_bool(value, key)?;
            Ok(())
        }
        "features.redis" => {
            manifest.features.redis = parse_bool(value, key)?;
            Ok(())
        }
        "features.email" => {
            manifest.features.email = parse_bool(value, key)?;
            Ok(())
        }
        "features.storage" => {
            manifest.features.storage = parse_bool(value, key)?;
            Ok(())
        }
        "features.audit_log" => {
            manifest.features.audit_log = parse_bool(value, key)?;
            Ok(())
        }
        "features.rate_limit" => {
            manifest.features.rate_limit = parse_bool(value, key)?;
            Ok(())
        }
        "features.idempotency" => {
            manifest.features.idempotency = parse_bool(value, key)?;
            Ok(())
        }
        "features.observability" => {
            manifest.features.observability = parse_bool(value, key)?;
            Ok(())
        }
        "features.docker" => {
            manifest.features.docker = parse_bool(value, key)?;
            Ok(())
        }
        "features.cicd" => {
            manifest.features.cicd = parse_bool(value, key)?;
            Ok(())
        }
        _ => Err(CliError::InvalidInput(format!("invalid config key: {key}"))),
    }
}

fn parse_bool(value: &str, key: &str) -> Result<bool, CliError> {
    match value.to_lowercase().as_str() {
        "true" | "1" | "yes" => Ok(true),
        "false" | "0" | "no" => Ok(false),
        _ => Err(CliError::InvalidInput(format!(
            "invalid value for {key}: expected true or false"
        ))),
    }
}

fn parse_enum<T>(value: &str, key: &str) -> Result<T, CliError>
where
    T: serde::de::DeserializeOwned,
{
    // Use TOML scalar parsing to honor serde rename attributes.
    let input = format!("value = \"{value}\"");
    #[derive(serde::Deserialize)]
    struct Wrapper<T> {
        value: T,
    }
    let wrapper: Wrapper<T> = toml::from_str(&input)
        .map_err(|_| CliError::InvalidInput(format!("invalid value for {key}: {value}")))?;
    Ok(wrapper.value)
}

fn serde_variant_name<T>(value: &T) -> String
where
    T: serde::Serialize,
{
    toml::to_string(value)
        .unwrap_or_default()
        .trim()
        .trim_matches('"')
        .to_string()
}
