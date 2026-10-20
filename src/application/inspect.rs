use crate::cli::Output;
use crate::cli::{DepsArgs, DoctorArgs, InfoArgs, ModulesArgs, RoutesArgs};
use crate::error::CliError;
use crate::inspection::InspectionContext;
use std::path::PathBuf;

fn resolve_start(path: &Option<PathBuf>) -> PathBuf {
    path.clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
}

pub fn info(args: &InfoArgs, output: &Output) -> Result<(), CliError> {
    let ctx = InspectionContext::build(&resolve_start(&args.path))?;
    crate::inspection::info::execute(&ctx, output, args.json)
}

pub fn modules(args: &ModulesArgs, output: &Output) -> Result<(), CliError> {
    let ctx = InspectionContext::build(&resolve_start(&args.path))?;
    crate::inspection::modules::execute(&ctx, output, args.json)
}

pub fn routes(args: &RoutesArgs, output: &Output) -> Result<(), CliError> {
    let ctx = InspectionContext::build(&resolve_start(&args.path))?;
    crate::inspection::routes::execute(&ctx, output, args.json)
}

pub fn deps(args: &DepsArgs, output: &Output) -> Result<(), CliError> {
    let ctx = InspectionContext::build(&resolve_start(&args.path))?;
    crate::inspection::deps::execute(&ctx, output, args.json)
}

pub fn doctor(args: &DoctorArgs, output: &Output) -> Result<(), CliError> {
    let ctx = InspectionContext::build(&resolve_start(&args.path))?;
    crate::inspection::doctor::execute(&ctx, output, args.json)
}
