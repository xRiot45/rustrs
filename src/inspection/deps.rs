use crate::cli::Output;
use crate::error::CliError;
use crate::inspection::InspectionContext;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Clone, Serialize)]
pub struct DependencyInfo {
    pub name: String,
    pub version: Option<String>,
    pub optional: bool,
    pub features: Vec<String>,
    pub section: String,
}

pub fn discover_deps(ctx: &InspectionContext) -> Vec<DependencyInfo> {
    let mut deps = Vec::new();
    let Some(cargo) = &ctx.cargo_manifest else {
        return deps;
    };

    for (section, key) in [
        ("dependencies", "dependencies"),
        ("dev-dependencies", "dev-dependencies"),
        ("build-dependencies", "build-dependencies"),
    ] {
        if let Some(table) = cargo.get(key).and_then(|v| v.as_table()) {
            for (name, value) in table {
                let (version, optional, features) = parse_dep_value(value);
                deps.push(DependencyInfo {
                    name: name.clone(),
                    version,
                    optional,
                    features,
                    section: section.to_string(),
                });
            }
        }
    }

    deps.sort_by(|a, b| a.name.cmp(&b.name).then(a.section.cmp(&b.section)));
    deps
}

fn parse_dep_value(value: &toml::Value) -> (Option<String>, bool, Vec<String>) {
    match value {
        toml::Value::String(v) => (Some(v.clone()), false, Vec::new()),
        toml::Value::Table(t) => {
            let version = t
                .get("version")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let optional = t.get("optional").and_then(|v| v.as_bool()).unwrap_or(false);
            let features = t
                .get("features")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();
            (version, optional, features)
        }
        _ => (None, false, Vec::new()),
    }
}

pub fn execute(ctx: &InspectionContext, output: &Output, json: bool) -> Result<(), CliError> {
    let deps = discover_deps(ctx);
    if json {
        let value = json!({ "dependencies": deps });
        output.info(&serde_json::to_string_pretty(&value).unwrap_or_default());
        return Ok(());
    }
    if deps.is_empty() {
        output.info("No dependencies declared.");
        return Ok(());
    }
    for d in &deps {
        let version = d.version.clone().unwrap_or_else(|| "*".to_string());
        let optional = if d.optional { " (optional)" } else { "" };
        let features = if d.features.is_empty() {
            String::new()
        } else {
            format!(" features={}", d.features.join(","))
        };
        output.info(&format!(
            "{} {}{}{} [{}]",
            d.name, version, optional, features, d.section
        ));
    }
    Ok(())
}
