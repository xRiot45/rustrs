use crate::error::CliError;
use crate::filesystem::paths::validate_relative_path;
use crate::generation::plan::{GenerationPlan, PlannedOperation};
use crate::generator::context::GeneratorContext;
use crate::generator::registration::{insert_between_markers, marker_pair};
use crate::template::renderer::render;
use std::path::PathBuf;

const EVENT_RS: &str = r#"use tokio::sync::broadcast;

pub type {{ singular_type }}Event = String;

pub fn channel() -> broadcast::Sender<{{ singular_type }}Event> {
    broadcast::channel(100).0
}

pub fn publish(tx: &broadcast::Sender<{{ singular_type }}Event>, event: {{ singular_type }}Event) {
    let _ = tx.send(event);
}

pub async fn handle_{{ module }}_event(event: {{ singular_type }}Event) {
    println!("handled event: {}", event);
}
"#;

pub fn plan(ctx: &GeneratorContext) -> Result<GenerationPlan, CliError> {
    let gen_ctx = ctx.gen_context();
    let file_path = format!("src/{}_event.rs", ctx.names.file_stem);
    let reg_path = "src/main.rs".to_string();
    let reg_line = format!("mod {}_event;", ctx.names.file_stem);

    let relative = validate_relative_path(PathBuf::from(&file_path).as_path())
        .map_err(|error| CliError::InvalidInput(error.to_string()))?;
    let contents = render(EVENT_RS, &gen_ctx)
        .map_err(CliError::Template)?
        .into_bytes();

    let mut operations = vec![PlannedOperation {
        relative_path: relative,
        contents,
        description: format!("generate event {}", ctx.names.singular_type),
        overwrite: false,
    }];

    let (begin, end) = marker_pair("modules");
    let reg_relative = validate_relative_path(PathBuf::from(&reg_path).as_path())
        .map_err(|error| CliError::InvalidInput(error.to_string()))?;
    let existing = std::fs::read_to_string(ctx.project_root.join(&reg_path)).ok();
    let updated = insert_between_markers(existing.as_deref(), &begin, &end, &reg_line)?;
    operations.push(PlannedOperation {
        relative_path: reg_relative,
        contents: updated.into_bytes(),
        description: format!("register event {}", ctx.names.singular_type),
        overwrite: true,
    });

    Ok(GenerationPlan {
        target_root: ctx.project_root.clone(),
        operations,
    })
}
