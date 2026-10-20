use crate::cli::Cli;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Output {
    pub quiet: bool,
    pub verbose: bool,
    pub no_color: bool,
}

impl Output {
    pub fn from_cli(cli: &Cli) -> Self {
        Self {
            quiet: cli.quiet,
            verbose: cli.verbose,
            no_color: cli.no_color,
        }
    }

    #[allow(dead_code)]
    pub fn info(&self, message: &str) {
        if !self.quiet {
            println!("{}", message);
        }
    }

    #[allow(dead_code)]
    pub fn warn(&self, message: &str) {
        eprintln!("warning: {}", message);
    }

    pub fn error(&self, message: &str) {
        eprintln!("error: {}", message);
    }
}
