use rustrs::cli::{Cli, Output};

#[test]
fn output_captures_flags() {
    let cli = Cli {
        verbose: true,
        quiet: false,
        no_color: true,
        command: None,
    };
    let output = Output::from_cli(&cli);
    assert!(output.verbose);
    assert!(!output.quiet);
    assert!(output.no_color);
}
