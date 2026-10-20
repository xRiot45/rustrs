use rustrs::compatibility::{CompatibilityStatus, check_compatibility, compatibility_status};
use rustrs::manifest::{
    Capability, Features, ManifestError, SUPPORTED_SCHEMA_VERSION, parse_manifest,
    validate_project_name, validate_schema_version,
};

const MINIMAL_VALID: &str = r#"
schema_version = 1
project_name = "demo"
template = "minimal"
framework = "axum"
architecture = "clean_modular"
database = "postgres"
data_access = "sqlx"
"#;

const COMPLETE_VALID: &str = r#"
schema_version = 1
project_name = "ecommerce-api"
template = "enterprise"
framework = "axum"
architecture = "clean_modular"
database = "postgres"
data_access = "sqlx"

[features]
auth = true
rbac = true
openapi = true
redis = false
email = false
storage = false
audit_log = true
rate_limit = false
idempotency = false
observability = true
docker = true
cicd = true
"#;

#[test]
fn parses_minimal_valid_manifest() {
    let manifest = parse_manifest(MINIMAL_VALID).unwrap();
    assert_eq!(manifest.schema_version, 1);
    assert_eq!(manifest.project_name, "demo");
    assert_eq!(manifest.template, rustrs::manifest::TemplateId::Minimal);
    assert_eq!(manifest.framework, rustrs::manifest::Framework::Axum);
    assert_eq!(
        manifest.architecture,
        rustrs::manifest::Architecture::CleanModular
    );
    assert_eq!(manifest.database, rustrs::manifest::Database::Postgres);
    assert_eq!(manifest.data_access, rustrs::manifest::DataAccess::Sqlx);
    manifest.validate().unwrap();
}

#[test]
fn parses_complete_valid_manifest() {
    let manifest = parse_manifest(COMPLETE_VALID).unwrap();
    assert!(manifest.features.auth);
    assert!(manifest.features.rbac);
    assert!(manifest.features.is_enabled(Capability::Rbac));
    manifest.validate().unwrap();
}

#[test]
fn rejects_malformed_toml() {
    let err = parse_manifest("schema_version =").unwrap_err();
    assert!(matches!(err, ManifestError::Parse { .. }));
}

#[test]
fn rejects_wrong_field_type() {
    let input = MINIMAL_VALID.replace("schema_version = 1", "schema_version = \"1\"");
    let err = parse_manifest(&input).unwrap_err();
    assert!(matches!(err, ManifestError::Parse { .. }));
}

#[test]
fn rejects_missing_required_field() {
    let input = MINIMAL_VALID.replace("project_name = \"demo\"\n", "");
    let err = parse_manifest(&input).unwrap_err();
    assert!(matches!(err, ManifestError::Parse { .. }));
}

#[test]
fn rejects_unknown_field() {
    let input = format!("{MINIMAL_VALID}\nunknown_field = true");
    let err = parse_manifest(&input).unwrap_err();
    assert!(matches!(err, ManifestError::Parse { .. }));
}

#[test]
fn supports_schema_version_one() {
    assert_eq!(SUPPORTED_SCHEMA_VERSION, 1);
    validate_schema_version(1).unwrap();
}

#[test]
fn rejects_unsupported_schema_version() {
    let err = validate_schema_version(2).unwrap_err();
    assert!(matches!(
        err,
        ManifestError::UnsupportedSchemaVersion { version: 2 }
    ));
}

#[test]
fn rejects_invalid_project_name() {
    let err = validate_project_name("Demo").unwrap_err();
    assert!(matches!(err, ManifestError::Validation { .. }));

    let err = validate_project_name("demo_app").unwrap_err();
    assert!(matches!(err, ManifestError::Validation { .. }));

    let err = validate_project_name("-demo").unwrap_err();
    assert!(matches!(err, ManifestError::Validation { .. }));
}

#[test]
fn accepts_valid_project_name() {
    validate_project_name("demo").unwrap();
    validate_project_name("ecommerce-api").unwrap();
    validate_project_name("demo2").unwrap();
}

#[test]
fn supported_combination_is_tier_one() {
    let manifest = parse_manifest(MINIMAL_VALID).unwrap();
    assert_eq!(
        compatibility_status(&manifest),
        CompatibilityStatus::Supported
    );
    check_compatibility(&manifest).unwrap();
}

#[test]
fn unsupported_combination_is_rejected() {
    let input = MINIMAL_VALID
        .replace("framework = \"axum\"", "framework = \"rocket\"")
        .replace("database = \"postgres\"", "database = \"mysql\"")
        .replace("data_access = \"sqlx\"", "data_access = \"diesel\"");
    let manifest = parse_manifest(&input).unwrap();
    let status = compatibility_status(&manifest);
    assert!(matches!(status, CompatibilityStatus::Unsupported { .. }));
    let err = check_compatibility(&manifest).unwrap_err();
    assert!(err.to_string().contains("unsupported combination"));
}

#[test]
fn capability_dependency_requires_auth() {
    let features = Features {
        rbac: true,
        ..Features::default()
    };
    let err = features.validate().unwrap_err();
    assert!(matches!(err, ManifestError::Capability(_)));

    let features = Features {
        auth: true,
        rbac: true,
        ..Features::default()
    };
    features.validate().unwrap();
}

#[test]
fn manifest_validation_is_separate_from_parsing() {
    let input = MINIMAL_VALID.replace("schema_version = 1", "schema_version = 2");
    let manifest = parse_manifest(&input).unwrap();
    let err = manifest.validate().unwrap_err();
    assert!(matches!(
        err,
        ManifestError::UnsupportedSchemaVersion { version: 2 }
    ));
}

#[test]
fn parse_does_not_validate_combination() {
    let input = MINIMAL_VALID.replace("framework = \"axum\"", "framework = \"rocket\"");
    let manifest = parse_manifest(&input).unwrap();
    assert_eq!(manifest.framework, rustrs::manifest::Framework::Rocket);
}

#[test]
fn project_manifest_rejects_unknown_fields() {
    let input = format!("{MINIMAL_VALID}\nextra = 1");
    let err = parse_manifest(&input).unwrap_err();
    assert!(matches!(err, ManifestError::Parse { .. }));
}
