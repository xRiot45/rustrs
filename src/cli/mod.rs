pub mod commands;
pub mod output;

pub use commands::{
    AuditArgs, CheckArgs, CompletionArgs, CreateMigrationArgs, DbArgs, DbCommands, DepsArgs,
    DevArgs, DevCommands, DevLifecycleArgs, DocsArgs, DoctorArgs, FormatArgs, GenerateArgs,
    InfoArgs, LintArgs, MigrateArgs, ModulesArgs, NewArgs, RollbackArgs, RoutesArgs, SeedArgs,
    StatusArgs, TestArgs,
};
pub use output::Output;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "rustrs", version, about = "Rust API scaffolding CLI")]
pub struct Cli {
    #[arg(short, long, global = true, conflicts_with = "quiet")]
    pub verbose: bool,

    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,

    #[arg(long, global = true)]
    pub no_color: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new Rust REST API project
    New(NewArgs),

    /// Generate code in an existing rustrs project
    #[command(alias = "g")]
    Generate(GenerateArgs),

    /// Inspect project configuration
    Info(InfoArgs),

    /// Inspect project modules
    Modules(ModulesArgs),

    /// Inspect project routes
    Routes(RoutesArgs),

    /// Inspect project dependencies
    Deps(DepsArgs),

    /// Run project diagnostics
    Doctor(DoctorArgs),

    /// Database workflow commands
    Db(DbArgs),

    /// Development workflow commands
    Dev(DevArgs),

    /// Run Cargo checks
    Check(CheckArgs),

    /// Run Cargo Clippy
    Lint(LintArgs),

    /// Run Cargo tests
    Test(TestArgs),

    /// Format Rust code
    Format(FormatArgs),

    /// Run dependency audit
    Audit(AuditArgs),

    /// Inspect OpenAPI documentation configuration
    Docs(DocsArgs),

    /// Generate shell completion
    Completion(CompletionArgs),
}
