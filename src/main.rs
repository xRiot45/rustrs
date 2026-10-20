use clap::Parser;
use rustrs::cli::{Cli, Output};
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

fn run(_cli: &Cli, _output: &Output) -> Result<(), CliError> {
    Ok(())
}
