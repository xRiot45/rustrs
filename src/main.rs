use clap::Parser;
use rustrs::application;
use rustrs::cli::{Cli, Commands, Output};
use rustrs::error::CliError;
use std::process;

fn main() {
    let cli = Cli::parse();
    let output = Output::from_cli(&cli);

    if let Err(err) = run(&cli, &output) {
        output.error(&err.to_string());
        process::exit(err.exit_code());
    }
}

fn run(cli: &Cli, output: &Output) -> Result<(), CliError> {
    match &cli.command {
        Some(Commands::New(args)) => application::new_project::execute(args, output),
        None => Ok(()),
    }
}
