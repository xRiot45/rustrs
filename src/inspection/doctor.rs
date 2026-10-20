use crate::cli::Output;
use crate::error::CliError;
use crate::inspection::environment::environment_findings;
use crate::inspection::structure::check_required_files;
use crate::inspection::{Finding, InspectionContext, Severity};
use serde_json::json;

pub fn execute(ctx: &InspectionContext, output: &Output, json: bool) -> Result<(), CliError> {
    let mut findings: Vec<Finding> = ctx.findings.clone();

    // Compatibility.
    if let Some(crate::compatibility::CompatibilityStatus::Unsupported {
        reason,
        alternatives,
    }) = &ctx.compatibility
    {
        findings.push(Finding {
            code: "COMPATIBILITY_UNSUPPORTED".to_string(),
            severity: Severity::Error,
            title: "Unsupported technology combination".to_string(),
            detail: Some(reason.clone()),
            path: ctx.manifest_path.clone(),
            suggestion: Some(format!("Use one of: {}", alternatives.join(", "))),
            certain: true,
        });
    }

    // Required files.
    findings.extend(check_required_files(ctx));

    // Environment.
    findings.extend(environment_findings());

    let error_count = findings
        .iter()
        .filter(|f| f.severity == Severity::Error)
        .count();
    let warning_count = findings
        .iter()
        .filter(|f| f.severity == Severity::Warning)
        .count();

    if json {
        let value = json!({
            "findings": findings,
            "errors": error_count,
            "warnings": warning_count,
        });
        output.info(&serde_json::to_string_pretty(&value).unwrap_or_default());
    } else {
        output.info("rustrs doctor");
        output.info("");
        for f in &findings {
            let status = match f.severity {
                Severity::Error => "[ERROR]",
                Severity::Warning => "[WARN]",
                Severity::Info => "[INFO]",
            };
            output.info(&format!("{} {} - {}", status, f.code, f.title));
            if let Some(detail) = &f.detail {
                output.info(&format!("       {}", detail));
            }
            if let Some(suggestion) = &f.suggestion {
                output.info(&format!("       suggestion: {}", suggestion));
            }
        }
        output.info("");
        output.info(&format!("Errors: {}", error_count));
        output.info(&format!("Warnings: {}", warning_count));
    }

    if error_count > 0 {
        return Err(CliError::Diagnostic("doctor found errors".to_string()));
    }
    Ok(())
}
