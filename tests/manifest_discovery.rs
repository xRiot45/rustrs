use rustrs::manifest::{ManifestError, discover_manifest};
use std::fs;
use tempfile::TempDir;

const VALID_MANIFEST: &str = r#"
schema_version = 1
project_name = "demo"
template = "minimal"
framework = "axum"
architecture = "clean_modular"
database = "postgres"
data_access = "sqlx"
"#;

#[test]
fn discovers_manifest_in_current_directory() {
    let dir = TempDir::new().unwrap();
    let rustrs_dir = dir.path().join(".rustrs");
    fs::create_dir(&rustrs_dir).unwrap();
    fs::write(rustrs_dir.join("config.toml"), VALID_MANIFEST).unwrap();

    let discovered = discover_manifest(dir.path()).unwrap();
    assert_eq!(discovered.project_root, dir.path());
    assert_eq!(discovered.manifest_path, rustrs_dir.join("config.toml"));
    assert_eq!(discovered.manifest.project_name, "demo");
}

#[test]
fn discovers_manifest_from_nested_directory() {
    let dir = TempDir::new().unwrap();
    let rustrs_dir = dir.path().join(".rustrs");
    fs::create_dir(&rustrs_dir).unwrap();
    fs::write(rustrs_dir.join("config.toml"), VALID_MANIFEST).unwrap();
    let nested = dir.path().join("src").join("nested");
    fs::create_dir_all(&nested).unwrap();

    let discovered = discover_manifest(&nested).unwrap();
    assert_eq!(discovered.project_root, dir.path());
}

#[test]
fn cargo_project_without_manifest_is_not_managed() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .unwrap();

    let err = discover_manifest(dir.path()).unwrap_err();
    assert!(matches!(err, ManifestError::NotFound { .. }));
}

#[test]
fn missing_manifest_returns_not_found() {
    let dir = TempDir::new().unwrap();
    let err = discover_manifest(dir.path()).unwrap_err();
    assert!(matches!(err, ManifestError::NotFound { .. }));
}

#[test]
fn invalid_manifest_returns_parse_error() {
    let dir = TempDir::new().unwrap();
    let rustrs_dir = dir.path().join(".rustrs");
    fs::create_dir(&rustrs_dir).unwrap();
    fs::write(rustrs_dir.join("config.toml"), "schema_version =").unwrap();

    let err = discover_manifest(dir.path()).unwrap_err();
    assert!(matches!(err, ManifestError::Parse { .. }));
}

#[test]
fn discovery_does_not_create_files() {
    let dir = TempDir::new().unwrap();
    let before: Vec<_> = fs::read_dir(dir.path()).unwrap().collect();
    let _ = discover_manifest(dir.path());
    let after: Vec<_> = fs::read_dir(dir.path()).unwrap().collect();
    assert_eq!(before.len(), after.len());
}

#[test]
fn unreadable_or_invalid_start_path_returns_invalid_path() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("not-a-dir.txt");
    fs::write(&file, "").unwrap();

    let err = discover_manifest(&file).unwrap_err();
    assert!(matches!(err, ManifestError::InvalidPath { .. }));
}
