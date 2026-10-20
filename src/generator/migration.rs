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
            "migration generator requires a database configuration".to_string(),
        ));
    }

    let gen_ctx = ctx.gen_context();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let file_path = format!("migrations/{}_{}.sql", timestamp, ctx.names.module);

    let relative = validate_relative_path(PathBuf::from(&file_path).as_path())
        .map_err(|e| CliError::InvalidInput(e.to_string()))?;
    let contents = render(MIGRATION_SQL, &gen_ctx)
        .map_err(CliError::Template)?
        .into_bytes();

    Ok(GenerationPlan {
        target_root: ctx.project_root.clone(),
        operations: vec![PlannedOperation {
            relative_path: relative,
            contents,
            description: format!("generate migration {}", ctx.names.module),
            overwrite: false,
        }],
    })
}
