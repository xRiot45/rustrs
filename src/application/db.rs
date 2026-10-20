use crate::cli::{
    CreateMigrationArgs, DbArgs, DbCommands, MigrateArgs, RollbackArgs, SeedArgs, StatusArgs,
};
use crate::compatibility::check_compatibility;
use crate::error::CliError;
use crate::filesystem::apply_plan;
use crate::generation::plan::{OperationOutcome, WritePolicy};
use crate::generator::context::GeneratorContext;
use crate::generator::migration;
use crate::manifest::discovery::discover_manifest;
use crate::manifest::model::Database;
use crate::process::runner::{ProcessRunner, SystemProcessRunner};
use crate::process::spec::{ProcessError, ProcessSpec};
use crate::process::tools::require_tool;
use std::env;
use std::path::PathBuf;

pub fn execute(args: &DbArgs, output: &crate::cli::Output) -> Result<(), CliError> {
    match &args.command {
        DbCommands::CreateMigration(a) => create_migration(a, output),
        DbCommands::Migrate(a) => migrate(a, output),
        DbCommands::Rollback(a) => rollback(a, output),
        DbCommands::Status(a) => status(a, output),
        DbCommands::Seed(a) => seed(a, output),
    }
}

fn resolve_project(
    path: Option<PathBuf>,
) -> Result<(PathBuf, crate::manifest::model::ProjectManifest), CliError> {
    let start = path.unwrap_or_else(|| env::current_dir().unwrap_or_default());
    let discovered = discover_manifest(&start)?;
    discovered.manifest.validate()?;
    check_compatibility(&discovered.manifest)?;
    if discovered.manifest.database != Database::Postgres
        || discovered.manifest.data_access != crate::manifest::model::DataAccess::Sqlx
    {
        return Err(CliError::InvalidInput(
            "database workflow is only supported for Axum + Postgres + SQLx".to_string(),
        ));
    }
    Ok((discovered.project_root, discovered.manifest))
}

fn create_migration(
    args: &CreateMigrationArgs,
    output: &crate::cli::Output,
) -> Result<(), CliError> {
    let (root, manifest) = resolve_project(args.path.clone())?;
    if manifest.database == Database::None {
        return Err(CliError::InvalidInput(
            "migration generation requires a database configuration".to_string(),
        ));
    }

    let input = crate::domain::NameInput {
        name: args.name.clone(),
        singular: None,
        plural: None,
    };
    let names = crate::domain::normalize_name(&input)?;

    let ctx = GeneratorContext {
        project_root: root.clone(),
        manifest,
        names,
        skip_tests: false,
        skip_docs: false,
    };

    let plan = migration::plan(&ctx)?;
    let report = apply_plan(&plan, WritePolicy::PreserveExisting);
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

    let mut failed = false;
    for result in &report.results {
        if matches!(result.outcome, OperationOutcome::Failed { .. }) {
            failed = true;
        }
    }
    if failed {
        return Err(CliError::InvalidInput(
            "migration creation failed for one or more files".to_string(),
        ));
    }

    if !output.quiet {
        output.info("Migration file created.");
    }
    Ok(())
}

fn run_sqlx(
    root: &std::path::Path,
    args: Vec<String>,
    output: &crate::cli::Output,
) -> Result<(), CliError> {
    require_tool("sqlx").map_err(|_| CliError::MissingTool("sqlx".to_string()))?;
    let mut spec = ProcessSpec::new("sqlx");
    spec.args = args;
    spec.cwd = Some(root.to_path_buf());

    let runner = SystemProcessRunner;
    let result = runner.run(spec).map_err(|error| match error {
        ProcessError::NotFound(name) => CliError::MissingTool(name),
        other => CliError::ExternalCommand(other.to_string()),
    })?;

    if !result.stdout.trim().is_empty() {
        output.info(&redact(&result.stdout));
    }
    if !result.stderr.trim().is_empty() {
        output.warn(&redact(&result.stderr));
    }

    if result.code != 0 {
        return Err(CliError::ExternalCommand(format!(
            "sqlx exited with status {}",
            result.code
        )));
    }
    Ok(())
}

fn migrate(args: &MigrateArgs, output: &crate::cli::Output) -> Result<(), CliError> {
    let (root, _) = resolve_project(args.path.clone())?;
    if args.dry_run {
        run_sqlx(
            &root,
            vec!["migrate".into(), "run".into(), "--dry-run".into()],
            output,
        )
    } else {
        run_sqlx(&root, vec!["migrate".into(), "run".into()], output)
    }
}

fn rollback(args: &RollbackArgs, output: &crate::cli::Output) -> Result<(), CliError> {
    let (root, _) = resolve_project(args.path.clone())?;
    let mut command = vec!["migrate".to_string(), "revert".to_string()];
    if let Some(version) = args.target_version {
        command.push("--target-version".to_string());
        command.push(version.to_string());
    }
    run_sqlx(&root, command, output)
}

fn status(args: &StatusArgs, output: &crate::cli::Output) -> Result<(), CliError> {
    let (root, _) = resolve_project(args.path.clone())?;
    run_sqlx(&root, vec!["migrate".into(), "info".into()], output)
}

fn seed(_args: &SeedArgs, _output: &crate::cli::Output) -> Result<(), CliError> {
    Err(CliError::InvalidInput(
        "database seeding is not supported because no real seeder mechanism exists".to_string(),
    ))
}

fn redact(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    for line in text.lines() {
        result.push_str(&redact_line(line));
        result.push('\n');
    }
    result
}

fn redact_line(line: &str) -> String {
    if let Some(start) = line.find("://") {
        if let Some(at) = line[start..].find('@') {
            let at = start + at;
            if let Some(colon) = line[start + 3..at].rfind(':') {
                let colon = start + 3 + colon;
                return format!("{}{}***{}", &line[..colon + 1], "", &line[at..]);
            }
        }
    }
    line.to_string()
}
