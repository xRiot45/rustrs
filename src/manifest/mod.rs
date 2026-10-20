pub mod discovery;
pub mod error;
pub mod model;
pub mod parser;
pub mod validation;

pub use discovery::{DiscoveredManifest, discover_manifest};
pub use error::ManifestError;
pub use model::{
    Architecture, Capability, DataAccess, Database, Features, Framework, ProjectManifest,
    TemplateId,
};
pub use parser::parse_manifest;
pub use validation::{SUPPORTED_SCHEMA_VERSION, validate_project_name, validate_schema_version};
