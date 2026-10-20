use crate::manifest::model::{DataAccess, Database, Framework, TemplateId};
use clap::Args;

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
