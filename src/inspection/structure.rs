use crate::cli::Output;
use crate::inspection::{Finding, InspectionContext, Severity};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct StructureCheck {
    pub path: String,
    pub present: bool,
    pub required: bool,
}

pub fn check_required_files(ctx: &InspectionContext) -> Vec<Finding> {
    let mut findings = Vec::new();
    for rel in &ctx.required_files {
        let full = ctx.project_root.join(rel);
        if !full.is_file() {
            findings.push(Finding {
                code: "MISSING_REQUIRED_FILE".to_string(),
                severity: Severity::Error,
                title: format!("Required file is missing: {rel}"),
                detail: None,
                path: Some(full),
                suggestion: Some("Regenerate or restore the file.".to_string()),
                certain: true,
            });
        }
    }
    findings
}

pub fn required_file_statuses(ctx: &InspectionContext) -> Vec<StructureCheck> {
    ctx.required_files
        .iter()
        .map(|rel| StructureCheck {
            path: rel.clone(),
            present: ctx.project_root.join(rel).is_file(),
            required: true,
        })
        .collect()
}

pub fn execute(ctx: &InspectionContext, output: &Output, json: bool) {
    let statuses = required_file_statuses(ctx);
    if json {
        output.info(&serde_json::to_string_pretty(&statuses).unwrap_or_default());
        return;
    }
    for s in &statuses {
        if s.present {
            output.info(&format!("[OK] {}", s.path));
        } else {
            output.warn(&format!("[MISSING] {}", s.path));
        }
    }
}
