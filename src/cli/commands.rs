use crate::generator::kind::GeneratorKind;
use crate::manifest::model::{DataAccess, Database, Framework, TemplateId};
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct NewArgs {
    pub project_name: String,

    #[arg(long)]
    pub template: Option<TemplateId>,

    #[arg(long)]
    pub framework: Option<Framework>,

    #[arg(long)]
    pub database: Option<Database>,

    #[arg(long, name = "orm")]
    pub data_access: Option<DataAccess>,

    #[arg(long)]
    pub non_interactive: bool,

    #[arg(long)]
    pub defaults: bool,

    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Args, Debug)]
pub struct GenerateArgs {
    pub generator: GeneratorKind,

    pub name: String,

    #[arg(long)]
    pub singular: Option<String>,

    #[arg(long)]
    pub plural: Option<String>,

    #[arg(long)]
    pub dry_run: bool,

    #[arg(long)]
    pub force: bool,

    #[arg(long)]
    pub skip_tests: bool,

    #[arg(long)]
    pub skip_docs: bool,

    #[arg(long)]
    pub path: Option<PathBuf>,
}
