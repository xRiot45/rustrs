use crate::error::CliError;
use crate::filesystem::paths::validate_relative_path;
use crate::generation::plan::{GenerationPlan, PlannedOperation};
use crate::generator::context::GeneratorContext;
use crate::generator::registration::{insert_between_markers, marker_pair};
use crate::generator::templates::DTO_RS;
use crate::template::renderer::render;
use std::path::PathBuf;

pub fn plan(ctx: &GeneratorContext) -> Result<GenerationPlan, CliError> {
    let module = &ctx.names.module;
    let gen_ctx = ctx.gen_context();

    let (file_path, reg_path, reg_line) = if ctx.is_minimal() || ctx.is_simple() {
        (
            format!("src/{}_dto.rs", ctx.names.file_stem),
            "src/main.rs".to_string(),
            format!("mod {}_dto;", ctx.names.file_stem),
        )
    } else {
        (
            format!("src/modules/{module}/dto.rs"),
            format!("src/modules/{module}/mod.rs"),
            "pub mod dto;".to_string(),
        )
    };

    let relative = validate_relative_path(PathBuf::from(&file_path).as_path())
        .map_err(|e| CliError::InvalidInput(e.to_string()))?;
    let contents = render(DTO_RS, &gen_ctx)
        .map_err(CliError::Template)?
        .into_bytes();

    let mut operations = vec![PlannedOperation {
        relative_path: relative,
        contents,
        description: format!("generate dto {}", ctx.names.singular_type),
        overwrite: false,
    }];

    let (begin, end) = marker_pair("modules");
    let reg_relative = validate_relative_path(PathBuf::from(&reg_path).as_path())
        .map_err(|e| CliError::InvalidInput(e.to_string()))?;
    let existing = std::fs::read_to_string(ctx.project_root.join(&reg_path)).ok();
    let updated = insert_between_markers(existing.as_deref(), &begin, &end, &reg_line)?;
    operations.push(PlannedOperation {
        relative_path: reg_relative,
        contents: updated.into_bytes(),
        description: format!("register dto {}", ctx.names.singular_type),
        overwrite: true,
    });

    Ok(GenerationPlan {
        target_root: ctx.project_root.clone(),
        operations,
    })
}
