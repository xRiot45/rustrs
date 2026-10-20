pub mod commands;
pub mod output;

pub use commands::NewArgs;
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
}
