use clap::Parser;
use rustrs::application;
use rustrs::application::completion;
use rustrs::application::db;
use rustrs::application::dev;
use rustrs::application::docs;
use rustrs::application::generate;
use rustrs::application::inspect;
use rustrs::application::quality;
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
        Some(Commands::Generate(args)) => generate::execute(args, output),
        Some(Commands::Info(args)) => inspect::info(args, output),
        Some(Commands::Modules(args)) => inspect::modules(args, output),
        Some(Commands::Routes(args)) => inspect::routes(args, output),
        Some(Commands::Deps(args)) => inspect::deps(args, output),
        Some(Commands::Doctor(args)) => inspect::doctor(args, output),
        Some(Commands::Db(args)) => db::execute(args, output),
        Some(Commands::Dev(args)) => dev::execute(args, output),
        Some(Commands::Check(args)) => quality::check(args, output),
        Some(Commands::Lint(args)) => quality::lint(args, output),
        Some(Commands::Test(args)) => quality::test(args, output),
        Some(Commands::Format(args)) => quality::format(args, output),
        Some(Commands::Audit(args)) => quality::audit(args, output),
        Some(Commands::Docs(args)) => docs::execute(args, output),
        Some(Commands::Completion(args)) => completion::execute(args, output),
        None => Ok(()),
    }
}
