use crate::error::CliError;
use crate::filesystem::paths::validate_relative_path;
use crate::generation::plan::{GenerationPlan, PlannedOperation};
use crate::generator::context::GeneratorContext;
use crate::generator::registration::{insert_between_markers, marker_pair};
use crate::template::renderer::render;
use std::path::PathBuf;

const GUARD_RS: &str = r#"use axum::{http::StatusCode, middleware::Next, response::Response, http::Request};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::Deserialize;

#[derive(Deserialize)]
struct GuardClaims {
    sub: String,
    exp: usize,
    token_type: String,
}

pub async fn {{ module }}_guard(
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "));

    let token = token.ok_or(StatusCode::UNAUTHORIZED)?;
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "change-me-in-production".to_string());

    let data = decode::<GuardClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if data.claims.token_type != "access" {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}
"#;

pub fn plan(ctx: &GeneratorContext) -> Result<GenerationPlan, CliError> {
    let gen_ctx = ctx.gen_context();

    let (file_path, reg_path, reg_line) = (
        "src/guard.rs".to_string(),
        "src/main.rs".to_string(),
        "mod guard;".to_string(),
    );

    let relative = validate_relative_path(PathBuf::from(&file_path).as_path())
        .map_err(|error| CliError::InvalidInput(error.to_string()))?;
    let contents = render(GUARD_RS, &gen_ctx)
        .map_err(CliError::Template)?
        .into_bytes();

    let mut operations = vec![PlannedOperation {
        relative_path: relative,
        contents,
        description: format!("generate guard {}", ctx.names.singular_type),
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
        description: format!("register guard {}", ctx.names.singular_type),
        overwrite: true,
    });

    Ok(GenerationPlan {
        target_root: ctx.project_root.clone(),
        operations,
    })
}
