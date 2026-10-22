use crate::cli::{Output, UpdateArgs, UpdateCheckArgs, UpdateCommands};
use crate::error::CliError;
use std::process::Command;

pub fn execute(args: &UpdateArgs, output: &Output) -> Result<(), CliError> {
    match &args.command {
        UpdateCommands::Check(check) => check_update(check, output),
    }
}

fn check_update(args: &UpdateCheckArgs, output: &Output) -> Result<(), CliError> {
    let result = Command::new("cargo")
        .args(["search", "rustrs", "--limit", "1"])
        .output()
        .map_err(|error| CliError::ExternalCommand(error.to_string()))?;

    if !result.status.success() {
        return Err(CliError::ExternalCommand(
            String::from_utf8_lossy(&result.stderr).to_string(),
        ));
    }

    let stdout = String::from_utf8_lossy(&result.stdout);
    let latest = stdout
        .lines()
        .next()
        .and_then(|line| line.split('"').nth(1))
        .unwrap_or("unknown");

    let current = env!("CARGO_PKG_VERSION");

    if args.json {
        let value = serde_json::json!({
            "current": current,
            "latest": latest,
            "update_available": latest != current && latest != "unknown",
        });
        output.info(&serde_json::to_string_pretty(&value).unwrap_or_default());
    } else if latest == "unknown" {
        output.info("Could not determine the latest version.");
    } else if latest == current {
        output.info(&format!("rustrs is up to date ({current})."));
    } else {
        output.info(&format!(
            "Update available: {current} -> {latest}. Run `cargo install --locked rustrs` to update."
        ));
    }

    Ok(())
}
