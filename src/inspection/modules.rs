use crate::cli::Output;
use crate::error::CliError;
use crate::inspection::InspectionContext;
use serde_json::json;
use std::fs;

#[derive(Debug, Clone, serde::Serialize)]
pub struct DiscoveredModule {
    pub name: String,
    pub path: std::path::PathBuf,
}

pub fn discover_modules(ctx: &InspectionContext) -> Vec<DiscoveredModule> {
    let mut modules = Vec::new();
    let template = ctx.manifest.as_ref().map(|m| m.template);

    match template {
        Some(crate::manifest::model::TemplateId::Standard)
        | Some(crate::manifest::model::TemplateId::Enterprise) => {
            let modules_dir = ctx.project_root.join("src").join("modules");
            if let Ok(entries) = fs::read_dir(&modules_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() && path.join("mod.rs").is_file() {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            modules.push(DiscoveredModule {
                                name: name.to_string(),
                                path: path.join("mod.rs"),
                            });
                        }
                    } else if path.is_file() && path.extension().map(|e| e == "rs").unwrap_or(false)
                    {
                        if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                            if name != "mod" {
                                modules.push(DiscoveredModule {
                                    name: name.to_string(),
                                    path: path.clone(),
                                });
                            }
                        }
                    }
                }
            }
        }
        Some(crate::manifest::model::TemplateId::Minimal)
        | Some(crate::manifest::model::TemplateId::Simple) => {
            // Marker-aware discovery: only modules declared in the rustrs module marker.
            let main_path = ctx.project_root.join("src").join("main.rs");
            let mut names = Vec::new();
            if let Ok(content) = fs::read_to_string(&main_path) {
                let mut in_block = false;
                for line in content.lines() {
                    if line.contains("rustrs:begin:modules") {
                        in_block = true;
                        continue;
                    }
                    if line.contains("rustrs:end:modules") {
                        in_block = false;
                        continue;
                    }
                    if in_block {
                        let trimmed = line.trim();
                        // Accept `mod name;` and `mod name`
                        if let Some(rest) = trimmed.strip_prefix("mod ") {
                            let name = rest.trim().trim_end_matches(';').trim();
                            if !name.is_empty()
                                && name.chars().all(|c| c.is_alphanumeric() || c == '_')
                            {
                                names.push(name.to_string());
                            }
                        }
                    }
                }
            }

            if names.is_empty() {
                // Fallback for projects without markers.
                let src_dir = ctx.project_root.join("src");
                if let Ok(entries) = fs::read_dir(&src_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_file() && path.extension().map(|e| e == "rs").unwrap_or(false) {
                            if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                                if name != "main" {
                                    names.push(name.to_string());
                                }
                            }
                        }
                    }
                }
            }

            for name in names {
                let path = ctx.project_root.join("src").join(format!("{name}.rs"));
                if path.is_file() {
                    modules.push(DiscoveredModule { name, path });
                }
            }
        }
        None => {}
    }

    modules.sort_by(|a, b| a.name.cmp(&b.name));
    modules
}

pub fn execute(ctx: &InspectionContext, output: &Output, json: bool) -> Result<(), CliError> {
    let modules = discover_modules(ctx);
    if json {
        let value = json!({ "modules": modules });
        output.info(&serde_json::to_string_pretty(&value).unwrap_or_default());
        return Ok(());
    }
    if modules.is_empty() {
        output.info("No modules discovered.");
        return Ok(());
    }
    for m in &modules {
        output.info(&format!("{}  {}", m.name, m.path.display()));
    }
    Ok(())
}
