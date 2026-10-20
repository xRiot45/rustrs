pub const MODULE_MOD_RS: &str =
    "// {{ module }} module\n\n// rustrs:begin:modules\n// rustrs:end:modules\n";

pub const ENTITY_RS: &str = r#"#[derive(Debug, Clone)]
pub struct {{ singular_type }} {
    pub id: i64,
}
"#;

pub const DTO_RS: &str = r#"use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Create{{ singular_type }}Request {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{ singular_type }}Response {}
"#;

pub const SERVICE_RS: &str = r#"pub struct {{ singular_type }}Service;

impl {{ singular_type }}Service {
    pub fn new() -> Self {
        Self
    }
}
"#;

pub const REPOSITORY_RS: &str = r#"pub trait {{ singular_type }}Repository {}

pub struct Sqlx{{ singular_type }}Repository;

impl {{ singular_type }}Repository for Sqlx{{ singular_type }}Repository {}
"#;

pub const HANDLER_RS: &str = r#"use axum::Json;

pub async fn list_{{ route_segment }}() -> Json<&'static str> {
    Json("ok")
}
"#;

pub const MIDDLEWARE_RS: &str = r#"use axum::{body::Body, http::Request, middleware::Next, response::Response};

pub async fn {{ module }}_middleware(req: Request<Body>, next: Next) -> Response {
    next.run(req).await
}
"#;

pub const TEST_RS: &str = r#"#[cfg(test)]
mod tests {
    #[test]
    fn placeholder() {}
}
"#;

pub const MIGRATION_SQL: &str = "-- migration: {{ module }}\n";
