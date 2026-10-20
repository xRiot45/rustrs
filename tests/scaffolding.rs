use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn rustrs() -> Command {
    Command::cargo_bin("rustrs").unwrap()
}

#[test]
fn new_help_reflects_contract() {
    rustrs()
        .args(["new", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("PROJECT_NAME"))
        .stdout(predicate::str::contains("--template"))
        .stdout(predicate::str::contains("--dry-run"))
        .stdout(predicate::str::contains("--template"));
}

#[test]
fn new_invalid_name_fails() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "123bad"])
        .assert()
        .failure();
}

#[test]
fn new_incomplete_noninteractive_fails() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--non-interactive"])
        .assert()
        .failure();
}

#[test]
fn new_defaults_creates_project() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--defaults"])
        .assert()
        .success();

    let root = dir.path().join("demo");
    assert!(root.join("Cargo.toml").is_file());
    assert!(root.join("src/main.rs").is_file());
    assert!(root.join("README.md").is_file());
    assert!(root.join(".rustrs/config.toml").is_file());

    let manifest = fs::read_to_string(root.join(".rustrs/config.toml")).unwrap();
    assert!(manifest.contains("project_name = \"demo\""));
    assert!(manifest.contains("template = \"minimal\""));
}

#[test]
fn new_dry_run_does_not_create_files() {
    let dir = TempDir::new().unwrap();
    let before: Vec<_> = fs::read_dir(dir.path()).unwrap().collect();

    rustrs()
        .current_dir(dir.path())
        .args(["new", "drydemo", "--defaults", "--dry-run"])
        .assert()
        .success();

    let after: Vec<_> = fs::read_dir(dir.path()).unwrap().collect();
    assert_eq!(before.len(), after.len());
    assert!(!dir.path().join("drydemo").exists());
}

#[test]
fn new_existing_target_conflict_fails() {
    let dir = TempDir::new().unwrap();
    fs::create_dir_all(dir.path().join("demo")).unwrap();
    fs::write(dir.path().join("demo/file.txt"), "x").unwrap();

    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--defaults"])
        .assert()
        .failure()
        .code(3);
}

#[test]
fn new_all_templates() {
    let dir = TempDir::new().unwrap();
    for template in ["minimal", "simple", "standard", "enterprise"] {
        let name = format!("proj-{}", template);
        rustrs()
            .current_dir(dir.path())
            .args(["new", &name, "--template", template, "--defaults"])
            .assert()
            .success();
        let root = dir.path().join(&name);
        assert!(root.join("Cargo.toml").is_file());
        assert!(root.join("src/main.rs").is_file());
        assert!(root.join(".rustrs/config.toml").is_file());
    }
}

#[test]
fn new_unsupported_combo_fails() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo2", "--defaults", "--framework", "rocket"])
        .assert()
        .failure()
        .code(2);
}

#[test]
fn new_global_options_accepted() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["--no-color", "new", "demo3", "--defaults"])
        .assert()
        .success();
}
