use crate::error::CliError;
use crate::filesystem::paths::validate_relative_path;
use crate::generation::plan::{GenerationPlan, PlannedOperation};
use crate::generator::context::GeneratorContext;
use crate::generator::templates::TEST_RS;
use crate::template::renderer::render;
use std::path::PathBuf;

pub fn plan(ctx: &GeneratorContext) -> Result<GenerationPlan, CliError> {
    let gen_ctx = ctx.gen_context();

    // Test file location.
    let file_path = if ctx.is_minimal() {
        format!("tests/{}_tests.rs", ctx.names.file_stem)
    } else if ctx.is_simple() {
        format!("src/{}_tests.rs", ctx.names.file_stem)
    } else {
        format!("src/modules/{}/tests.rs", ctx.names.module)
    };

    let relative = validate_relative_path(PathBuf::from(&file_path).as_path())
        .map_err(|e| CliError::InvalidInput(e.to_string()))?;
    let contents = render(TEST_RS, &gen_ctx)
        .map_err(CliError::Template)?
        .into_bytes();

    let mut operations = vec![PlannedOperation {
        relative_path: relative,
        contents,
        description: format!("generate test {}", ctx.names.singular_type),
        overwrite: false,
    }];

    // Register tests module for standard/enterprise.
    if !ctx.is_minimal() && !ctx.is_simple() {
        let reg_path = format!("src/modules/{}/mod.rs", ctx.names.module);
        let reg_relative = validate_relative_path(PathBuf::from(&reg_path).as_path())
            .map_err(|e| CliError::InvalidInput(e.to_string()))?;
        let existing = std::fs::read_to_string(ctx.project_root.join(&reg_path)).ok();
        let updated = crate::generator::registration::insert_between_markers(
            existing.as_deref(),
            "// rustrs:begin:modules",
            "// rustrs:end:modules",
            "#[cfg(test)]\npub mod tests;",
        )?;
        operations.push(PlannedOperation {
            relative_path: reg_relative,
            contents: updated.into_bytes(),
            description: format!("register tests {}", ctx.names.singular_type),
            overwrite: true,
        });
    }

    Ok(GenerationPlan {
        target_root: ctx.project_root.clone(),
        operations,
    })
}
