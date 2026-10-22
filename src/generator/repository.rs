const REPOSITORY_SEAORM_RS: &str = r#"// Minimal SeaORM repository placeholder for {{ singular_type }}
pub trait {{ singular_type }}Repository {
    fn find(&self, id: &str) -> Option<String>;
}

pub struct SeaOrm{{ singular_type }}Repository;

impl {{ singular_type }}Repository for SeaOrm{{ singular_type }}Repository {
    fn find(&self, _id: &str) -> Option<String> {
        None
    }
}
"#;

const REPOSITORY_DIESEL_RS: &str = r#"// Minimal Diesel repository placeholder for {{ singular_type }}
pub trait {{ singular_type }}Repository {
    fn find(&self, id: &str) -> Option<String>;
}

pub struct Diesel{{ singular_type }}Repository;

impl {{ singular_type }}Repository for Diesel{{ singular_type }}Repository {
    fn find(&self, _id: &str) -> Option<String> {
        None
    }
}
"#;

use crate::error::CliError;
use crate::filesystem::paths::validate_relative_path;
use crate::generation::plan::{GenerationPlan, PlannedOperation};
use crate::generator::context::GeneratorContext;
use crate::generator::registration::{insert_between_markers, marker_pair};
use crate::generator::templates::REPOSITORY_RS;
use crate::template::renderer::render;
use std::path::PathBuf;

pub fn plan(ctx: &GeneratorContext) -> Result<GenerationPlan, CliError> {
    if !ctx.requires_database() {
        return Err(CliError::InvalidInput(
            "repository generator requires a database/data-access configuration".to_string(),
        ));
    }
    let module = &ctx.names.module;
    let gen_ctx = ctx.gen_context();

    let (file_path, reg_path, reg_line) = if ctx.is_minimal() || ctx.is_simple() {
        (
            format!("src/{}_repository.rs", ctx.names.file_stem),
            "src/main.rs".to_string(),
            format!("mod {}_repository;", ctx.names.file_stem),
        )
    } else {
        (
            format!("src/modules/{module}/repository.rs"),
            format!("src/modules/{module}/mod.rs"),
            "pub mod repository;".to_string(),
        )
    };

    let relative = validate_relative_path(PathBuf::from(&file_path).as_path())
        .map_err(|e| CliError::InvalidInput(e.to_string()))?;
    let repository_template = match ctx.manifest.data_access {
        crate::manifest::model::DataAccess::Sqlx => REPOSITORY_RS,
        crate::manifest::model::DataAccess::SeaOrm => REPOSITORY_SEAORM_RS,
        crate::manifest::model::DataAccess::Diesel => REPOSITORY_DIESEL_RS,
        crate::manifest::model::DataAccess::None => {
            return Err(CliError::InvalidInput(
                "repository generator requires a data-access configuration".to_string(),
            ));
        }
    };
    let contents = render(repository_template, &gen_ctx)
        .map_err(CliError::Template)?
        .into_bytes();

    let mut operations = vec![PlannedOperation {
        relative_path: relative,
        contents,
        description: format!("generate repository {}", ctx.names.singular_type),
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
        description: format!("register repository {}", ctx.names.singular_type),
        overwrite: true,
    });

    Ok(GenerationPlan {
        target_root: ctx.project_root.clone(),
        operations,
    })
}
