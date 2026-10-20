use crate::cli::Cli;
use crate::cli::CompletionArgs;
use crate::error::CliError;
use clap::CommandFactory;
use std::io;

pub fn execute(args: &CompletionArgs, output: &crate::cli::Output) -> Result<(), CliError> {
    let mut command = Cli::command();
    clap_complete::generate(args.shell, &mut command, "rustrs", &mut io::stdout());
    let _ = output;
    Ok(())
}
