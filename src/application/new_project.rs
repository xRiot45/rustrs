use crate::cli::NewArgs;
use crate::cli::Output;
use crate::compatibility::check_compatibility;
use crate::error::CliError;
use crate::filesystem::apply_plan;
use crate::filesystem::report::preview_plan;
use crate::generation::plan::{OperationOutcome, WritePolicy};
use crate::generation::planner::build_plan;
use crate::manifest::discovery::discover_manifest;
use crate::manifest::model::{
    Architecture, DataAccess, Database, Features, Framework, ProjectManifest, TemplateId,
};
use crate::manifest::validation::validate_project_name;
use crate::process::tools::require_tool;
use crate::template::metadata::TemplateError;
use dialoguer::{Confirm, Select};
use std::fs;
use std::io::IsTerminal;
use std::path::PathBuf;
use std::process::Command;

pub struct ResolvedScaffold {
    pub project_name: String,
    pub manifest: ProjectManifest,
    pub target_root: PathBuf,
    pub interactive: bool,
}

pub fn execute(args: &NewArgs, output: &Output) -> Result<(), CliError> {
    let resolved = resolve_scaffold(args)?;

    if !output.quiet {
        output.info(&format!("Project: {}", resolved.project_name));
        output.info(&format!("Output:  {}", resolved.target_root.display()));
        output.info(&format!(
            "Template: {}",
            template_name(resolved.manifest.template)
        ));
        output.info(&format!(
            "Framework: {}",
            framework_name(resolved.manifest.framework)
        ));
        output.info(&format!(
            "Database: {} ({})",
            database_name(resolved.manifest.database),
            data_access_name(resolved.manifest.data_access)
        ));
    }

    if args.dry_run {
        let plan = build_plan(
            &resolved.manifest,
            resolved.manifest.template,
            &resolved.target_root,
        )?;
        let report = preview_plan(&plan);
        print_report(output, &report);
        return Ok(());
    }

    if resolved.interactive {
        let proceed = Confirm::new()
            .with_prompt("Create project?")
            .default(true)
            .interact()
            .map_err(|error| CliError::InvalidInput(error.to_string()))?;
        if !proceed {
            output.info("Cancelled.");
            return Ok(());
        }
    }

    if !resolved.target_root.exists() {
        fs::create_dir_all(&resolved.target_root)
            .map_err(|error| CliError::InvalidInput(error.to_string()))?;
    }

    let plan = build_plan(
        &resolved.manifest,
        resolved.manifest.template,
        &resolved.target_root,
    )?;
    let report = apply_plan(&plan, WritePolicy::PreserveExisting);
    print_report(output, &report);

    let mut failed = false;
    for result in &report.results {
        if matches!(result.outcome, OperationOutcome::Failed { .. }) {
            failed = true;
        }
    }
    if failed {
        return Err(CliError::InvalidInput(
            "scaffolding failed for one or more files".to_string(),
        ));
    }

    let discovered = discover_manifest(&resolved.target_root)?;
    discovered.manifest.validate()?;

    verify_structure(&resolved)?;

    if args.git {
        require_tool("git").map_err(|_| CliError::MissingTool("git".to_string()))?;
        let status = Command::new("git")
            .arg("init")
            .current_dir(&resolved.target_root)
            .status()
            .map_err(|error| CliError::ExternalCommand(error.to_string()))?;
        if !status.success() {
            return Err(CliError::ExternalCommand(format!(
                "git init failed with status {}",
                status.code().unwrap_or(-1)
            )));
        }
        if !output.quiet {
            output.info("Initialized empty Git repository.");
        }
    }

    if !output.quiet {
        output.info("");
        output.info("Next steps:");
        output.info(&format!("  cd {}", resolved.project_name));
        output.info("  cargo check");
    }

    Ok(())
}

fn resolve_scaffold(args: &NewArgs) -> Result<ResolvedScaffold, CliError> {
    let interactive = !args.non_interactive && std::io::stdin().is_terminal();

    validate_project_name(&args.project_name)?;

    let (template, framework, database, data_access) = resolve_inputs(args, interactive)?;

    let mut features = Features::default();
    if matches!(template, TemplateId::Enterprise | TemplateId::Standard) {
        features.email = true;
        features.storage = true;
        features.rate_limit = true;
        features.idempotency = true;
        features.observability = true;
        if database == Database::Postgres && data_access == DataAccess::Sqlx {
            features.auth = true;
            features.rbac = true;
            features.audit_log = true;
        }
    }

    let manifest = ProjectManifest {
        schema_version: 1,
        project_name: args.project_name.clone(),
        template,
        framework,
        architecture: Architecture::CleanModular,
        database,
        data_access,
        features,
    };

    manifest.validate()?;
    check_compatibility(&manifest)?;

    let target_root = std::env::current_dir()
        .map_err(|error| CliError::InvalidInput(error.to_string()))?
        .join(&args.project_name);

    if let Some(parent) = target_root.parent() {
        if !parent.exists() {
            return Err(CliError::InvalidInput(format!(
                "parent directory does not exist: {}",
                parent.display()
            )));
        }
    }

    if target_root.exists() {
        let is_empty_dir = target_root.is_dir()
            && fs::read_dir(&target_root)
                .map(|mut entries| entries.next().is_none())
                .unwrap_or(false);
        if !is_empty_dir {
            return Err(CliError::Conflict(format!(
                "target already exists: {}",
                target_root.display()
            )));
        }
    }

    Ok(ResolvedScaffold {
        project_name: args.project_name.clone(),
        manifest,
        target_root,
        interactive,
    })
}

fn resolve_inputs(
    args: &NewArgs,
    interactive: bool,
) -> Result<(TemplateId, Framework, Database, DataAccess), CliError> {
    let template = match args.template {
        Some(value) => value,
        None => {
            if args.defaults {
                TemplateId::Minimal
            } else if interactive {
                let labels = ["minimal", "simple", "standard", "enterprise"];
                let selection = Select::new()
                    .with_prompt("Template")
                    .items(&labels)
                    .default(0)
                    .interact()
                    .map_err(|error| CliError::InvalidInput(error.to_string()))?;
                match selection {
                    0 => TemplateId::Minimal,
                    1 => TemplateId::Simple,
                    2 => TemplateId::Standard,
                    _ => TemplateId::Enterprise,
                }
            } else {
                return Err(CliError::InvalidInput(
                    "missing --template; use --defaults or provide flags (example: rustrs new my-api --template minimal --defaults)".to_string(),
                ));
            }
        }
    };

    let framework = match args.framework {
        Some(value) => value,
        None => {
            if args.defaults || interactive {
                Framework::Axum
            } else {
                return Err(CliError::InvalidInput(
                    "missing --framework; use --defaults or provide flags".to_string(),
                ));
            }
        }
    };

    let database = match args.database {
        Some(value) => value,
        None => {
            if args.defaults || interactive {
                Database::Postgres
            } else {
                return Err(CliError::InvalidInput(
                    "missing --database; use --defaults or provide flags".to_string(),
                ));
            }
        }
    };

    let data_access = match args.data_access {
        Some(value) => value,
        None => {
            if args.defaults || interactive {
                DataAccess::Sqlx
            } else {
                return Err(CliError::InvalidInput(
                    "missing --orm; use --defaults or provide flags".to_string(),
                ));
            }
        }
    };

    Ok((template, framework, database, data_access))
}

fn print_report(output: &Output, report: &crate::generation::OperationReport) {
    for result in &report.results {
        let line = match &result.outcome {
            OperationOutcome::Planned => format!("  PLAN   {}", result.relative_path.display()),
            OperationOutcome::Created => format!("  CREATE {}", result.relative_path.display()),
            OperationOutcome::Skipped { reason } => {
                format!("  SKIP   {} ({})", result.relative_path.display(), reason)
            }
            OperationOutcome::Failed { error } => {
                format!("  FAIL   {} ({})", result.relative_path.display(), error)
            }
        };
        if !output.quiet {
            output.info(&line);
        }
    }
}

fn verify_structure(resolved: &ResolvedScaffold) -> Result<(), CliError> {
    let root = &resolved.target_root;
    for relative in [
        "Cargo.toml",
        "src/main.rs",
        "README.md",
        ".rustrs/config.toml",
    ] {
        if !root.join(relative).is_file() {
            return Err(CliError::InvalidInput(format!(
                "missing generated file: {}",
                relative
            )));
        }
    }

    for relative in ["Cargo.toml", "src/main.rs", "README.md"] {
        let contents = fs::read_to_string(root.join(relative))
            .map_err(|error| CliError::InvalidInput(error.to_string()))?;
        if contents.contains("{{") {
            return Err(CliError::InvalidInput(format!(
                "unresolved template placeholder in {}",
                relative
            )));
        }
    }

    Ok(())
}

fn template_name(value: TemplateId) -> &'static str {
    match value {
        TemplateId::Enterprise => "enterprise",
        TemplateId::Standard => "standard",
        TemplateId::Simple => "simple",
        TemplateId::Minimal => "minimal",
    }
}

fn framework_name(value: Framework) -> &'static str {
    match value {
        Framework::Axum => "axum",
        Framework::ActixWeb => "actix-web",
        Framework::Rocket => "rocket",
    }
}

fn database_name(value: Database) -> &'static str {
    match value {
        Database::Postgres => "postgres",
        Database::Mysql => "mysql",
        Database::Sqlite => "sqlite",
        Database::None => "none",
    }
}

fn data_access_name(value: DataAccess) -> &'static str {
    match value {
        DataAccess::Sqlx => "sqlx",
        DataAccess::SeaOrm => "sea-orm",
        DataAccess::Diesel => "diesel",
        DataAccess::None => "none",
    }
}

// Suppress unused import warning for TemplateError if not referenced.
#[allow(dead_code)]
fn _use_template_error(_: TemplateError) {}
