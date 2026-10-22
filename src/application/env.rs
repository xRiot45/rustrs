use crate::cli::{EnvArgs, EnvCheckArgs, EnvCommands, Output};
use crate::error::CliError;
use crate::inspection::environment::check_tool;
use crate::manifest::discovery::discover_manifest;
use std::env;

pub fn execute(args: &EnvArgs, output: &Output) -> Result<(), CliError> {
    match &args.command {
        EnvCommands::Check(check) => check_env(check, output),
    }
}

fn check_env(args: &EnvCheckArgs, output: &Output) -> Result<(), CliError> {
    let start = args
        .path
        .clone()
        .unwrap_or_else(|| env::current_dir().unwrap_or_default());

    let mut checks: Vec<EnvCheck> = Vec::new();

    let discovered = discover_manifest(&start).ok();
    if let Some(discovered) = &discovered {
        discovered.manifest.validate()?;
    }

    for tool in ["cargo", "rustc", "sqlx"] {
        let status = check_tool(tool);
        checks.push(EnvCheck {
            name: tool.to_string(),
            status: if status.available { "PASS" } else { "WARN" }.to_string(),
            detail: status.version,
        });
    }

    for tool in ["cargo-audit", "docker"] {
        let status = check_tool(tool);
        checks.push(EnvCheck {
            name: tool.to_string(),
            status: if status.available { "PASS" } else { "SKIP" }.to_string(),
            detail: if status.available {
                status.version
            } else {
                Some("optional tool not available".to_string())
            },
        });
    }

    if let Some(value) = env::var_os("DATABASE_URL") {
        let value = value.to_string_lossy().to_string();
        let valid = value.starts_with("postgres://") || value.starts_with("postgresql://");
        checks.push(EnvCheck {
            name: "DATABASE_URL".to_string(),
            status: if valid { "PASS" } else { "WARN" }.to_string(),
            detail: Some(if valid {
                "format looks valid".to_string()
            } else {
                "expected postgres:// or postgresql:// URL".to_string()
            }),
        });
    } else {
        checks.push(EnvCheck {
            name: "DATABASE_URL".to_string(),
            status: "SKIP".to_string(),
            detail: Some(
                "not set; database commands may use .env or runtime configuration".to_string(),
            ),
        });
    }

    if args.json {
        output.info(&serde_json::to_string_pretty(&checks).unwrap_or_default());
    } else {
        output.info("rustrs env check");
        output.info("");
        for check in &checks {
            output.info(&format!(
                "[{}] {} {}",
                check.status,
                check.name,
                check.detail.clone().unwrap_or_default()
            ));
        }
    }

    Ok(())
}

#[derive(serde::Serialize)]
struct EnvCheck {
    name: String,
    status: String,
    detail: Option<String>,
}
