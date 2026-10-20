use crate::cli::GenerateArgs;
use crate::cli::Output;
use crate::compatibility::check_compatibility;
use crate::domain::{NameInput, normalize_name};
use crate::error::CliError;
use crate::filesystem::apply_plan;
use crate::filesystem::report::preview_plan;
use crate::generation::plan::{OperationOutcome, WritePolicy};
use crate::generator::context::GeneratorContext;
use crate::generator::registry::plan_for;
use crate::manifest::discovery::discover_manifest;
use std::env;

pub fn execute(args: &GenerateArgs, output: &Output) -> Result<(), CliError> {
    let start = args
        .path
        .clone()
        .unwrap_or_else(|| env::current_dir().unwrap_or_default());
    let discovered = discover_manifest(&start)?;
    discovered.manifest.validate()?;
    check_compatibility(&discovered.manifest)?;

    let input = NameInput {
        name: args.name.clone(),
        singular: args.singular.clone(),
        plural: args.plural.clone(),
    };
    let names = normalize_name(&input)?;

    let ctx = GeneratorContext {
        project_root: discovered.project_root.clone(),
        manifest: discovered.manifest.clone(),
        names,
        skip_tests: args.skip_tests,
        skip_docs: args.skip_docs,
    };

    if !output.quiet {
        output.info(&format!("Project: {}", ctx.project_root.display()));
        output.info(&format!("Generator: {}", args.generator));
        output.info(&format!("Name: {}", args.name));
    }

    let plan = plan_for(args.generator, &ctx)?;

    if args.dry_run {
        let report = preview_plan(&plan);
        print_report(output, &report);
        return Ok(());
    }

    let policy = if args.force {
        WritePolicy::Overwrite
    } else {
        WritePolicy::PreserveExisting
    };
    let report = apply_plan(&plan, policy);
    print_report(output, &report);

    let mut failed = false;
    for result in &report.results {
        if matches!(result.outcome, OperationOutcome::Failed { .. }) {
            failed = true;
        }
    }
    if failed {
        return Err(CliError::InvalidInput(
            "generation failed for one or more files".to_string(),
        ));
    }

    if !output.quiet {
        output.info("");
        output.info("Generation completed.");
    }

    Ok(())
}

fn print_report(output: &Output, report: &crate::generation::plan::OperationReport) {
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
