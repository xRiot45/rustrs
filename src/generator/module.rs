use crate::error::CliError;
use crate::filesystem::paths::validate_relative_path;
use crate::generation::plan::{GenerationPlan, PlannedOperation};
use crate::generator::context::GeneratorContext;
use crate::generator::registration::{insert_between_markers, marker_pair};
use crate::generator::templates::MODULE_MOD_RS;
use crate::template::renderer::render;
use std::path::PathBuf;

pub fn plan(ctx: &GeneratorContext) -> Result<GenerationPlan, CliError> {
    let module = &ctx.names.module;
    let gen_ctx = ctx.gen_context();

    // Determine file path and registration target based on template.
    let (file_path, registration_path, (begin, end), registration_line) =
        if ctx.is_minimal() || ctx.is_simple() {
            let path = format!("src/{module}.rs");
            let reg_path = "src/main.rs".to_string();
            let markers = marker_pair("modules");
            let line = format!("mod {module};");
            (path, reg_path, markers, line)
        } else {
            let path = format!("src/modules/{module}/mod.rs");
            let reg_path = "src/modules/mod.rs".to_string();
            let markers = marker_pair("modules");
            let line = format!("pub mod {module};");
            (path, reg_path, markers, line)
        };

    let relative = validate_relative_path(PathBuf::from(&file_path).as_path())
        .map_err(|e| CliError::InvalidInput(e.to_string()))?;
    let contents = render(MODULE_MOD_RS, &gen_ctx)
        .map_err(CliError::Template)?
        .into_bytes();

    let mut operations = vec![PlannedOperation {
        relative_path: relative,
        contents,
        description: format!("generate module {module}"),
        overwrite: false,
    }];

    // Registration edit (read existing file content unknown here; store as a planned full-file rewrite is not possible without reading).
    // We instead plan registration as a separate operation using current filesystem read at planning time.
    let reg_relative = validate_relative_path(PathBuf::from(&registration_path).as_path())
        .map_err(|e| CliError::InvalidInput(e.to_string()))?;

    let existing = std::fs::read_to_string(ctx.project_root.join(&registration_path)).ok();
    let updated = insert_between_markers(existing.as_deref(), &begin, &end, &registration_line)?;
    operations.push(PlannedOperation {
        relative_path: reg_relative,
        contents: updated.into_bytes(),
        description: format!("register module {module}"),
        overwrite: true,
    });

    Ok(GenerationPlan {
        target_root: ctx.project_root.clone(),
        operations,
    })
}
