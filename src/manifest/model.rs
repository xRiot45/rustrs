use crate::manifest::error::ManifestError;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum TemplateId {
    Enterprise,
    Standard,
    Simple,
    Minimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum Framework {
    Axum,
    ActixWeb,
    Rocket,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Database {
    Postgres,
    Mysql,
    Sqlite,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum DataAccess {
    Sqlx,
    SeaOrm,
    Diesel,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, ValueEnum)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum Capability {
    Auth,
    Rbac,
    #[value(name = "openapi")]
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

impl std::fmt::Display for Capability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Auth => "auth",
            Self::Rbac => "rbac",
            Self::OpenApi => "openapi",
            Self::Redis => "redis",
            Self::Email => "email",
            Self::Storage => "storage",
            Self::AuditLog => "audit_log",
            Self::RateLimit => "rate_limit",
            Self::Idempotency => "idempotency",
            Self::Observability => "observability",
            Self::Docker => "docker",
            Self::Cicd => "cicd",
        };
        write!(f, "{}", name)
    }
}

impl Capability {
    pub fn is_implemented(&self) -> bool {
        matches!(
            self,
            Self::Docker
                | Self::Cicd
                | Self::Auth
                | Self::Rbac
                | Self::OpenApi
                | Self::Redis
                | Self::Email
                | Self::Storage
                | Self::AuditLog
                | Self::RateLimit
                | Self::Idempotency
                | Self::Observability
        )
    }

    pub fn effect_summary(&self) -> &'static str {
        match self {
            Self::Docker => "generates Dockerfile and .dockerignore",
            Self::Cicd => "generates CI workflow files",
            Self::Auth => "generates authentication module",
            Self::Rbac => "generates permission-based RBAC support",
            Self::OpenApi => "generates OpenAPI runtime and Scalar UI",
            Self::Redis => "generates Redis client, cache, and rate limiting",
            Self::Email => "generates email service",
            Self::Storage => "generates storage service",
            Self::AuditLog => "generates audit log service",
            Self::RateLimit => "generates rate limiting middleware",
            Self::Idempotency => "generates idempotency middleware",
            Self::Observability => "generates observability setup",
        }
    }
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
