use crate::manifest::model::ProjectManifest;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TemplateContext {
    pub project_name: String,
    pub template: String,
    pub framework: String,
    pub architecture: String,
    pub database: String,
    pub data_access: String,
}

impl TemplateContext {
    pub fn from_manifest(manifest: &ProjectManifest) -> Self {
        Self {
            project_name: manifest.project_name.clone(),
            template: template_name(manifest.template),
            framework: framework_name(manifest.framework),
            architecture: architecture_name(manifest.architecture),
            database: database_name(manifest.database),
            data_access: data_access_name(manifest.data_access),
        }
    }
}

fn template_name(value: crate::manifest::model::TemplateId) -> String {
    match value {
        crate::manifest::model::TemplateId::Enterprise => "enterprise".to_string(),
        crate::manifest::model::TemplateId::Standard => "standard".to_string(),
        crate::manifest::model::TemplateId::Simple => "simple".to_string(),
        crate::manifest::model::TemplateId::Minimal => "minimal".to_string(),
    }
}

fn framework_name(value: crate::manifest::model::Framework) -> String {
    match value {
        crate::manifest::model::Framework::Axum => "axum".to_string(),
        crate::manifest::model::Framework::ActixWeb => "actix-web".to_string(),
        crate::manifest::model::Framework::Rocket => "rocket".to_string(),
    }
}

fn architecture_name(value: crate::manifest::model::Architecture) -> String {
    match value {
        crate::manifest::model::Architecture::CleanModular => "clean_modular".to_string(),
    }
}

fn database_name(value: crate::manifest::model::Database) -> String {
    match value {
        crate::manifest::model::Database::Postgres => "postgres".to_string(),
        crate::manifest::model::Database::Mysql => "mysql".to_string(),
        crate::manifest::model::Database::Sqlite => "sqlite".to_string(),
        crate::manifest::model::Database::None => "none".to_string(),
    }
}

fn data_access_name(value: crate::manifest::model::DataAccess) -> String {
    match value {
        crate::manifest::model::DataAccess::Sqlx => "sqlx".to_string(),
        crate::manifest::model::DataAccess::SeaOrm => "sea-orm".to_string(),
        crate::manifest::model::DataAccess::Diesel => "diesel".to_string(),
        crate::manifest::model::DataAccess::None => "none".to_string(),
    }
}
