use crate::manifest::error::ManifestError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateId {
    Enterprise,
    Standard,
    Simple,
    Minimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Framework {
    Axum,
    ActixWeb,
    Rocket,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Database {
    Postgres,
    Mysql,
    Sqlite,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataAccess {
    Sqlx,
    SeaOrm,
    Diesel,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Architecture {
    CleanModular,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Features {
    pub auth: bool,
    pub rbac: bool,
    pub openapi: bool,
    pub redis: bool,
    pub email: bool,
    pub storage: bool,
    pub audit_log: bool,
    pub rate_limit: bool,
    pub idempotency: bool,
    pub observability: bool,
    pub docker: bool,
    pub cicd: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capability {
    Auth,
    Rbac,
    OpenApi,
    Redis,
    Email,
    Storage,
    AuditLog,
    RateLimit,
    Idempotency,
    Observability,
    Docker,
    Cicd,
}

impl Features {
    pub fn validate(&self) -> Result<(), ManifestError> {
        crate::compatibility::capabilities::validate_capabilities(self)
            .map_err(ManifestError::Capability)
    }

    pub fn is_enabled(&self, capability: Capability) -> bool {
        match capability {
            Capability::Auth => self.auth,
            Capability::Rbac => self.rbac,
            Capability::OpenApi => self.openapi,
            Capability::Redis => self.redis,
            Capability::Email => self.email,
            Capability::Storage => self.storage,
            Capability::AuditLog => self.audit_log,
            Capability::RateLimit => self.rate_limit,
            Capability::Idempotency => self.idempotency,
            Capability::Observability => self.observability,
            Capability::Docker => self.docker,
            Capability::Cicd => self.cicd,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectManifest {
    pub schema_version: u32,
    pub project_name: String,
    pub template: TemplateId,
    pub framework: Framework,
    pub architecture: Architecture,
    pub database: Database,
    pub data_access: DataAccess,
    #[serde(default)]
    pub features: Features,
}
