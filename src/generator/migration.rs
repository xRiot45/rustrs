use crate::error::CliError;
use crate::filesystem::paths::validate_relative_path;
use crate::generation::plan::{GenerationPlan, PlannedOperation};
use crate::generator::context::GeneratorContext;
use crate::generator::templates::MIGRATION_SQL;
use crate::template::renderer::render;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn plan(ctx: &GeneratorContext) -> Result<GenerationPlan, CliError> {
    if !ctx.requires_database() {
        return Err(CliError::InvalidInput(
            "migration generation requires a database configuration".to_string(),
        ));
    }
    if ctx.manifest.data_access != crate::manifest::model::DataAccess::Sqlx {
        return Err(CliError::InvalidInput(
            "migration generation currently supports SQLx only".to_string(),
        ));
    }

    let gen_ctx = ctx.gen_context();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let slug = &ctx.names.module;
    let up_path = format!("migrations/{}_{}.up.sql", timestamp, slug);
    let down_path = format!("migrations/{}_{}.down.sql", timestamp, slug);

    let up_relative = validate_relative_path(PathBuf::from(&up_path).as_path())
        .map_err(|error| CliError::InvalidInput(error.to_string()))?;
    let up_contents = render(MIGRATION_SQL, &gen_ctx)
        .map_err(CliError::Template)?
        .into_bytes();

    let down_relative = validate_relative_path(PathBuf::from(&down_path).as_path())
        .map_err(|error| CliError::InvalidInput(error.to_string()))?;
    let down_contents = reverse_migration(&String::from_utf8_lossy(&up_contents)).into_bytes();

    Ok(GenerationPlan {
        target_root: ctx.project_root.clone(),
        operations: vec![
            PlannedOperation {
                relative_path: up_relative,
                contents: up_contents,
                description: format!("generate migration {}", slug),
                overwrite: false,
            },
            PlannedOperation {
                relative_path: down_relative,
                contents: down_contents,
                description: format!("generate down migration {}", slug),
                overwrite: false,
            },
        ],
    })
}

fn reverse_migration(up: &str) -> String {
    let mut down = String::new();
    down.push_str("-- down migration (auto-generated; review before use)\n");
    for line in up.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("--") {
            continue;
        }
        let upper = trimmed.to_uppercase();
        if upper.starts_with("CREATE TABLE") {
            if let Some(table) = table_name_after(trimmed, "TABLE") {
                down.push_str(&format!("DROP TABLE IF EXISTS {};\n", table));
            } else {
                down.push_str(&format!("-- TODO: manual reverse required: {}\n", trimmed));
            }
        } else if upper.starts_with("ALTER TABLE") && upper.contains("ADD COLUMN") {
            if let Some((table, column)) = alter_add_column(trimmed) {
                down.push_str(&format!(
                    "ALTER TABLE {} DROP COLUMN IF EXISTS {};\n",
                    table, column
                ));
            } else {
                down.push_str(&format!("-- TODO: manual reverse required: {}\n", trimmed));
            }
        } else if upper.starts_with("CREATE INDEX") {
            if let Some(index) = index_name(trimmed) {
                down.push_str(&format!("DROP INDEX IF EXISTS {};\n", index));
            } else {
                down.push_str(&format!("-- TODO: manual reverse required: {}\n", trimmed));
            }
        } else if upper.starts_with("ALTER TABLE") && upper.contains("ADD CONSTRAINT") {
            if let Some((table, constraint)) = alter_add_constraint(trimmed) {
                down.push_str(&format!(
                    "ALTER TABLE {} DROP CONSTRAINT IF EXISTS {};\n",
                    table, constraint
                ));
            } else {
                down.push_str(&format!("-- TODO: manual reverse required: {}\n", trimmed));
            }
        } else {
            down.push_str(&format!("-- TODO: manual reverse required: {}\n", trimmed));
        }
    }
    if down.lines().count() <= 1 {
        down.push_str("-- no reversible statements found\n");
    }
    down
}

fn table_name_after(input: &str, keyword: &str) -> Option<String> {
    let upper = input.to_uppercase();
    let pos = upper.find(keyword)?;
    let rest = input[pos + keyword.len()..].trim();
    rest.split_whitespace()
        .next()
        .map(|s| s.trim_end_matches('(').to_string())
}

fn alter_add_column(input: &str) -> Option<(String, String)> {
    let upper = input.to_uppercase();
    let table = table_name_after(input, "TABLE")?;
    let pos = upper.find("ADD COLUMN")?;
    let rest = input[pos + "ADD COLUMN".len()..].trim();
    let column = rest.split_whitespace().next()?.to_string();
    Some((table, column))
}

fn alter_add_constraint(input: &str) -> Option<(String, String)> {
    let table = table_name_after(input, "TABLE")?;
    let upper = input.to_uppercase();
    let pos = upper.find("ADD CONSTRAINT")?;
    let rest = input[pos + "ADD CONSTRAINT".len()..].trim();
    let constraint = rest.split_whitespace().next()?.to_string();
    Some((table, constraint))
}

fn index_name(input: &str) -> Option<String> {
    let upper = input.to_uppercase();
    let pos = upper.find("INDEX")?;
    let rest = input[pos + "INDEX".len()..].trim();
    // handle CREATE INDEX name ON ...
    rest.split_whitespace().next().map(|s| s.to_string())
}
