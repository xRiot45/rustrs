use crate::cli::{AuditArgs, CheckArgs, FormatArgs, LintArgs, TestArgs};
use crate::error::CliError;
use crate::process::runner::{ProcessRunner, SystemProcessRunner};
use crate::process::spec::ProcessSpec;
use crate::process::tools::require_tool;
use std::env;
use std::path::PathBuf;

fn resolve_root(path: Option<PathBuf>) -> Result<PathBuf, CliError> {
    let root = path.unwrap_or_else(|| env::current_dir().unwrap_or_default());
    if !root.join("Cargo.toml").is_file() {
        return Err(CliError::InvalidInput(format!(
            "Cargo.toml not found in {}",
            root.display()
        )));
    }
    Ok(root)
}

fn run_tool(
    program: &str,
    args: Vec<String>,
    root: &std::path::Path,
    output: &crate::cli::Output,
) -> Result<(), CliError> {
    require_tool(program).map_err(|_| CliError::MissingTool(program.to_string()))?;
    let mut spec = ProcessSpec::new(program);
    spec.args = args;
    spec.cwd = Some(root.to_path_buf());
    let runner = SystemProcessRunner;
    let result = runner.run(spec).map_err(|error| match error {
        crate::process::spec::ProcessError::NotFound(name) => CliError::MissingTool(name),
        other => CliError::ExternalCommand(other.to_string()),
    })?;
    if !result.stdout.trim().is_empty() {
        output.info(&result.stdout);
    }
    if !result.stderr.trim().is_empty() {
        output.warn(&result.stderr);
    }
    if result.code != 0 {
        return Err(CliError::ExternalCommand(format!(
            "{} exited with status {}",
            program, result.code
        )));
    }
    Ok(())
}

pub fn check(args: &CheckArgs, output: &crate::cli::Output) -> Result<(), CliError> {
    let root = resolve_root(args.path.clone())?;
    run_tool("cargo", vec!["check".to_string()], &root, output)
}

pub fn lint(args: &LintArgs, output: &crate::cli::Output) -> Result<(), CliError> {
    let root = resolve_root(args.path.clone())?;
    run_tool(
        "cargo",
        vec![
            "clippy".to_string(),
            "--all-targets".to_string(),
            "--".to_string(),
            "-D".to_string(),
            "warnings".to_string(),
        ],
        &root,
        output,
    )
}

pub fn test(args: &TestArgs, output: &crate::cli::Output) -> Result<(), CliError> {
    let root = resolve_root(args.path.clone())?;
    run_tool("cargo", vec!["test".to_string()], &root, output)
}

pub fn format(args: &FormatArgs, output: &crate::cli::Output) -> Result<(), CliError> {
    let root = resolve_root(args.path.clone())?;
    if args.write {
        run_tool("cargo", vec!["fmt".to_string()], &root, output)
    } else {
        run_tool(
            "cargo",
            vec!["fmt".to_string(), "--check".to_string()],
            &root,
            output,
        )
    }
}

pub fn audit(args: &AuditArgs, output: &crate::cli::Output) -> Result<(), CliError> {
    let root = resolve_root(args.path.clone())?;
    require_tool("cargo-audit").map_err(|_| CliError::MissingTool("cargo-audit".to_string()))?;
    run_tool("cargo", vec!["audit".to_string()], &root, output)
}
