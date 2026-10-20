use crate::cli::Output;
use crate::error::CliError;
use crate::inspection::InspectionContext;
use serde::Serialize;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize)]
pub struct RouteInfo {
    pub method: String,
    pub path: String,
    pub handler: Option<String>,
    pub source: PathBuf,
}

pub fn discover_routes(ctx: &InspectionContext) -> Vec<RouteInfo> {
    let mut routes = Vec::new();
    let framework = ctx.manifest.as_ref().map(|m| m.framework);
    let src_dir = ctx.project_root.join("src");
    walk_rs_files(&src_dir, &mut |path| {
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => return,
        };
        match framework {
            Some(crate::manifest::model::Framework::Axum) => {
                parse_axum_routes(&path, &content, &mut routes);
            }
            Some(crate::manifest::model::Framework::ActixWeb) => {
                parse_attribute_routes(&path, &content, &mut routes, "actix");
            }
            Some(crate::manifest::model::Framework::Rocket) => {
                parse_attribute_routes(&path, &content, &mut routes, "rocket");
            }
            None => {}
        }
    });
    routes.sort_by(|a, b| a.path.cmp(&b.path).then(a.method.cmp(&b.method)));
    routes
}

fn walk_rs_files(dir: &std::path::Path, cb: &mut dyn FnMut(PathBuf)) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk_rs_files(&path, cb);
            } else if path.extension().map(|e| e == "rs").unwrap_or(false) {
                cb(path);
            }
        }
    }
}

fn parse_axum_routes(source_path: &std::path::Path, content: &str, routes: &mut Vec<RouteInfo>) {
    // Extract marker blocks if present, otherwise whole file.
    let mut blocks = Vec::new();
    let mut in_block = false;
    let mut current = String::new();
    for line in content.lines() {
        if line.contains("rustrs:begin:routes") {
            in_block = true;
            current.clear();
            continue;
        }
        if line.contains("rustrs:end:routes") {
            in_block = false;
            blocks.push(current.clone());
            current.clear();
            continue;
        }
        if in_block {
            current.push_str(line);
            current.push('\n');
        }
    }
    if blocks.is_empty() {
        blocks.push(content.to_string());
    }

    for block in blocks {
        for line in block.lines() {
            let trimmed = line.trim();
            if !trimmed.contains(".route(") {
                continue;
            }
            // Extract path.
            let mut path_val = None;
            if let Some(start) = trimmed.find(".route(") {
                let rest = &trimmed[start + 7..];
                if let Some(q1) = rest.find('"') {
                    let after = &rest[q1 + 1..];
                    if let Some(q2) = after.find('"') {
                        path_val = Some(after[..q2].to_string());
                    }
                }
            }
            let Some(path) = path_val else {
                continue;
            };

            // Method and handler extraction.
            let mut method = "ANY".to_string();
            let mut handler = None;
            if let Some(route_idx) = trimmed.find(".route(") {
                let rest = &trimmed[route_idx + 7..];
                if let Some(comma) = rest.find(',') {
                    let after_comma = rest[comma + 1..].trim();
                    // Method may be `get`, `routing::get`, etc.
                    let method_part = after_comma.split('(').next().unwrap_or("").trim();
                    let method_ident = method_part
                        .rsplit("::")
                        .next()
                        .unwrap_or(method_part)
                        .trim();
                    // Remove trailing non-ident chars.
                    let method_ident: String = method_ident
                        .chars()
                        .take_while(|c| c.is_alphanumeric() || *c == '_')
                        .collect();
                    if !method_ident.is_empty() {
                        method = method_ident.to_uppercase();
                    }

                    // Handler is the balanced content of the first '(' ... ')'.
                    if let Some(open) = after_comma.find('(') {
                        let mut depth = 0usize;
                        let mut handler_end = None;
                        for (idx, ch) in after_comma[open..].char_indices() {
                            match ch {
                                '(' => depth += 1,
                                ')' => {
                                    depth -= 1;
                                    if depth == 0 {
                                        handler_end = Some(open + idx);
                                        break;
                                    }
                                }
                                _ => {}
                            }
                        }
                        if let Some(end_idx) = handler_end {
                            let h = after_comma[open + 1..end_idx].trim();
                            if !h.is_empty() {
                                handler = Some(h.to_string());
                            }
                        }
                    }
                }
            }

            routes.push(RouteInfo {
                method,
                path,
                handler,
                source: source_path.to_path_buf(),
            });
        }
    }
}

fn parse_attribute_routes(
    path: &std::path::Path,
    content: &str,
    routes: &mut Vec<RouteInfo>,
    _framework: &str,
) {
    let methods = ["get", "post", "put", "delete", "patch", "head", "options"];
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();
        // Find attribute like #[get("/path")] or #[route("/path", method="GET")]
        for method in methods {
            let attr = format!("#[{}(", method);
            if line.starts_with(&attr) {
                if let Some(q1) = line.find('"') {
                    let after = &line[q1 + 1..];
                    if let Some(q2) = after.find('"') {
                        let path_val = after[..q2].to_string();
                        // handler: next fn
                        let mut handler = None;
                        for next_line in &lines[i + 1..] {
                            let next = next_line.trim();
                            if next.starts_with("fn ") || next.contains(" fn ") {
                                // extract name
                                if let Some(fn_idx) = next.find("fn ") {
                                    let after_fn = &next[fn_idx + 3..];
                                    let mut name = String::new();
                                    for c in after_fn.chars() {
                                        if c.is_alphanumeric() || c == '_' {
                                            name.push(c);
                                        } else {
                                            break;
                                        }
                                    }
                                    if !name.is_empty() {
                                        handler = Some(name);
                                    }
                                }
                                break;
                            }
                            if next.starts_with("#[") {
                                // another attribute; stop
                                break;
                            }
                        }
                        routes.push(RouteInfo {
                            method: method.to_uppercase(),
                            path: path_val,
                            handler,
                            source: path.to_path_buf(),
                        });
                    }
                }
            }
        }
        i += 1;
    }
}

pub fn execute(ctx: &InspectionContext, output: &Output, json: bool) -> Result<(), CliError> {
    let routes = discover_routes(ctx);
    if json {
        let value = json!({ "routes": routes });
        output.info(&serde_json::to_string_pretty(&value).unwrap_or_default());
        return Ok(());
    }
    if routes.is_empty() {
        output.info("No routes discovered (or route analysis incomplete).");
        return Ok(());
    }
    for r in &routes {
        output.info(&format!(
            "{} {}  {} ({})",
            r.method,
            r.path,
            r.handler.clone().unwrap_or_else(|| "?".to_string()),
            r.source.display()
        ));
    }
    Ok(())
}
