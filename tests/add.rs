use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn rustrs() -> Command {
    Command::cargo_bin("rustrs").unwrap()
}

fn scaffold() -> (TempDir, std::path::PathBuf) {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--defaults"])
        .assert()
        .success();
    let root = dir.path().join("demo");
    (dir, root)
}

#[test]
fn add_help_works() {
    rustrs().args(["add", "--help"]).assert().success();
}

#[test]
fn add_docker_creates_assets() {
    let (_dir, root) = scaffold();
    rustrs()
        .current_dir(&root)
        .args(["add", "docker"])
        .assert()
        .success();
    assert!(root.join("Dockerfile").is_file());
    assert!(root.join(".dockerignore").is_file());
    let manifest = fs::read_to_string(root.join(".rustrs/config.toml")).unwrap();
    assert!(manifest.contains("docker = true"));
}

#[test]
fn add_docker_is_idempotent() {
    let (_dir, root) = scaffold();
    rustrs()
        .current_dir(&root)
        .args(["add", "docker"])
        .assert()
        .success();
    rustrs()
        .current_dir(&root)
        .args(["add", "docker"])
        .assert()
        .success()
        .stdout(predicate::str::contains("SKIP"));
}

#[test]
fn add_docker_dry_run_does_not_write() {
    let (_dir, root) = scaffold();
    let before_manifest = fs::read_to_string(root.join(".rustrs/config.toml")).unwrap();
    rustrs()
        .current_dir(&root)
        .args(["add", "docker", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("PLAN"));
    assert!(!root.join("Dockerfile").exists());
    let after_manifest = fs::read_to_string(root.join(".rustrs/config.toml")).unwrap();
    assert_eq!(before_manifest, after_manifest);
}

#[test]
fn add_docker_force_overwrites() {
    let (_dir, root) = scaffold();
    fs::write(root.join("Dockerfile"), "old").unwrap();
    rustrs()
        .current_dir(&root)
        .args(["add", "docker", "--force"])
        .assert()
        .success();
    let contents = fs::read_to_string(root.join("Dockerfile")).unwrap();
    assert!(contents.contains("FROM rust"));
}

#[test]
fn add_cicd_github_creates_workflow() {
    let (_dir, root) = scaffold();
    rustrs()
        .current_dir(&root)
        .args(["add", "cicd", "--provider", "github"])
        .assert()
        .success();
    assert!(root.join(".github/workflows/ci.yml").is_file());
    let manifest = fs::read_to_string(root.join(".rustrs/config.toml")).unwrap();
    assert!(manifest.contains("cicd = true"));
}

#[test]
fn add_cicd_gitlab_creates_workflow() {
    let (_dir, root) = scaffold();
    rustrs()
        .current_dir(&root)
        .args(["add", "cicd", "--provider", "gitlab"])
        .assert()
        .success();
    assert!(root.join(".gitlab-ci.yml").is_file());
}

#[test]
fn add_unsupported_capability_fails() {
    let (_dir, root) = scaffold();
    let before = fs::read_to_string(root.join(".rustrs/config.toml")).unwrap();
    rustrs()
        .current_dir(&root)
        .args(["add", "auth"])
        .assert()
        .failure()
        .code(2);
    let after = fs::read_to_string(root.join(".rustrs/config.toml")).unwrap();
    assert_eq!(before, after);
}

#[test]
fn add_outside_project_fails() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["add", "docker"])
        .assert()
        .failure();
}

#[test]
fn add_docker_conflict_without_force_skips() {
    let (_dir, root) = scaffold();
    fs::write(root.join("Dockerfile"), "old").unwrap();
    rustrs()
        .current_dir(&root)
        .args(["add", "docker"])
        .assert()
        .success()
        .stdout(predicate::str::contains("SKIP"));
}

#[test]
fn add_openapi_creates_module() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--template", "standard", "--defaults"])
        .assert()
        .success();
    let root = dir.path().join("demo");
    rustrs()
        .current_dir(&root)
        .args(["add", "openapi"])
        .assert()
        .success();
    assert!(root.join("src/modules/openapi.rs").is_file());
    let manifest = fs::read_to_string(root.join(".rustrs/config.toml")).unwrap();
    assert!(manifest.contains("openapi = true"));
}

#[test]
fn add_redis_creates_module() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--template", "standard", "--defaults"])
        .assert()
        .success();
    let root = dir.path().join("demo");
    rustrs()
        .current_dir(&root)
        .args(["add", "redis"])
        .assert()
        .success();
    assert!(root.join("src/modules/redis.rs").is_file());
    let manifest = fs::read_to_string(root.join(".rustrs/config.toml")).unwrap();
    assert!(manifest.contains("redis = true"));
}

#[test]
fn add_email_creates_module() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--template", "standard", "--defaults"])
        .assert()
        .success();
    let root = dir.path().join("demo");
    rustrs()
        .current_dir(&root)
        .args(["add", "email"])
        .assert()
        .success();
    assert!(root.join("src/modules/email.rs").is_file());
    let manifest = fs::read_to_string(root.join(".rustrs/config.toml")).unwrap();
    assert!(manifest.contains("email = true"));
}
