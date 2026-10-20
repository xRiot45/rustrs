use rustrs::project::{ProjectRootError, discover_project_root};
use std::fs;
use tempfile::TempDir;

#[test]
fn finds_root_in_start_directory() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("Cargo.toml"), "").unwrap();

    let found = discover_project_root(dir.path()).unwrap();
    assert_eq!(found, fs::canonicalize(dir.path()).unwrap());
}

#[test]
fn finds_root_from_nested_directory() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("Cargo.toml"), "").unwrap();
    let nested = dir.path().join("src");
    fs::create_dir(&nested).unwrap();

    let found = discover_project_root(&nested).unwrap();
    assert_eq!(found, fs::canonicalize(dir.path()).unwrap());
}

#[test]
fn returns_not_found_when_no_marker_exists() {
    let dir = TempDir::new().unwrap();
    let nested = dir.path().join("src");
    fs::create_dir(&nested).unwrap();

    let err = discover_project_root(&nested).unwrap_err();
    assert!(matches!(err, ProjectRootError::NotFound { .. }));
}

#[test]
fn returns_invalid_path_for_missing_start() {
    let dir = TempDir::new().unwrap();
    let missing = dir.path().join("does-not-exist");

    let err = discover_project_root(&missing).unwrap_err();
    assert!(matches!(err, ProjectRootError::InvalidPath { .. }));
}

#[test]
fn returns_invalid_path_for_file_start() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("file.txt");
    fs::write(&file, "").unwrap();

    let err = discover_project_root(&file).unwrap_err();
    assert!(matches!(err, ProjectRootError::InvalidPath { .. }));
}
