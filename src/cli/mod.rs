pub mod commands;
pub mod output;

pub use commands::{
    DepsArgs, DoctorArgs, GenerateArgs, InfoArgs, ModulesArgs, NewArgs, RoutesArgs,
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
}
