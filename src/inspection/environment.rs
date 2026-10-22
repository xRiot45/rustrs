use crate::cli::Output;
use crate::inspection::{Finding, Severity};
use serde::Serialize;
use std::process::Command;

#[derive(Debug, Clone, Serialize)]
pub struct ToolStatus {
    pub name: String,
    pub available: bool,
    pub version: Option<String>,
}

pub fn check_tool(name: &str) -> ToolStatus {
    let output = Command::new(name).arg("--version").output();
    match output {
        Ok(out) if out.status.success() => {
            let version = String::from_utf8_lossy(&out.stdout).trim().to_string();
            let version = if version.is_empty() {
                None
            } else {
                Some(version)
            };
            ToolStatus {
                name: name.to_string(),
                available: true,
                version,
            }
        }
        _ => ToolStatus {
            name: name.to_string(),
            available: false,
            version: None,
        },
    }
}

pub fn environment_findings() -> Vec<Finding> {
    let mut findings = Vec::new();
    for tool in ["cargo", "rustc"] {
        let status = check_tool(tool);
        if !status.available {
            findings.push(Finding {
                code: "TOOL_MISSING".to_string(),
                severity: Severity::Warning,
                title: format!("{tool} is not available"),
                detail: None,
                path: None,
                suggestion: Some(format!("Install {tool} and ensure it is on PATH.")),
                certain: true,
            });
        }
    }

    for tool in ["sqlx", "cargo-audit", "docker"] {
        let status = check_tool(tool);
        if !status.available {
            findings.push(Finding {
                code: "OPTIONAL_TOOL_MISSING".to_string(),
                severity: Severity::Info,
                title: format!("{tool} is not available"),
                detail: Some("optional tool".to_string()),
                path: None,
                suggestion: Some(format!(
                    "Install {tool} and ensure it is on PATH if you use the related workflow."
                )),
                certain: true,
            });
        }
    }

    findings
}

pub fn execute(output: &Output, json: bool) {
    let tools = vec![check_tool("cargo"), check_tool("rustc")];
    if json {
        output.info(&serde_json::to_string_pretty(&tools).unwrap_or_default());
        return;
    }
    for t in &tools {
        if t.available {
            output.info(&format!(
                "[OK] {} {}",
                t.name,
                t.version.clone().unwrap_or_default()
            ));
        } else {
            output.warn(&format!("[WARN] {} is not available", t.name));
        }
    }
}
