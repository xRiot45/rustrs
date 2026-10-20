use crate::generator::kind::GeneratorKind;
use crate::manifest::model::{DataAccess, Database, Framework, TemplateId};
use clap::Args;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct InfoArgs {
    #[arg(long)]
    pub path: Option<PathBuf>,

    #[arg(long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct ModulesArgs {
    #[arg(long)]
    pub path: Option<PathBuf>,

    #[arg(long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct RoutesArgs {
    #[arg(long)]
    pub path: Option<PathBuf>,

    #[arg(long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct DepsArgs {
    #[arg(long)]
    pub path: Option<PathBuf>,

    #[arg(long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct DoctorArgs {
    #[arg(long)]
    pub path: Option<PathBuf>,

    #[arg(long)]
    pub json: bool,
}

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

#[derive(Args, Debug)]
pub struct DbArgs {
    #[command(subcommand)]
    pub command: DbCommands,
}

#[derive(Subcommand, Debug)]
pub enum DbCommands {
    /// Create a new migration file
    CreateMigration(CreateMigrationArgs),

    /// Run pending migrations
    Migrate(MigrateArgs),

    /// Roll back the latest migration
    Rollback(RollbackArgs),

    /// Show migration status
    Status(StatusArgs),

    /// Run database seeders
    Seed(SeedArgs),
}

#[derive(Args, Debug)]
pub struct CreateMigrationArgs {
    pub name: String,

    #[arg(long)]
    pub path: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct MigrateArgs {
    #[arg(long)]
    pub dry_run: bool,

    #[arg(long)]
    pub path: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct RollbackArgs {
    #[arg(long)]
    pub target_version: Option<i64>,

    #[arg(long)]
    pub path: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct StatusArgs {
    #[arg(long)]
    pub path: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct SeedArgs {
    #[arg(long)]
    pub path: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct DevArgs {
    #[arg(long)]
    pub watch: bool,

    #[arg(long)]
    pub path: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<DevCommands>,
}

#[derive(Subcommand, Debug)]
pub enum DevCommands {
    /// Start the application in the background
    Up(DevLifecycleArgs),

    /// Stop the background application
    Down(DevLifecycleArgs),

    /// Show background application logs
    Logs(DevLifecycleArgs),
}

#[derive(Args, Debug)]
pub struct DevLifecycleArgs {
    #[arg(long)]
    pub path: Option<PathBuf>,
}
