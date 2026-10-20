use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

fn rustrs() -> Command {
    Command::cargo_bin("rustrs").unwrap()
}

fn scaffold_minimal() -> (TempDir, std::path::PathBuf) {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--template", "minimal", "--defaults"])
        .assert()
        .success();
    let root = dir.path().join("demo");
    (dir, root)
}

#[test]
fn phase8_command_help_works() {
    for args in [
        ["check", "--help"],
        ["lint", "--help"],
        ["test", "--help"],
        ["format", "--help"],
        ["audit", "--help"],
        ["docs", "--help"],
        ["completion", "--help"],
    ] {
        rustrs().args(args).assert().success();
    }
}

#[test]
fn completion_bash_contains_function() {
    rustrs()
        .args(["completion", "bash"])
        .assert()
        .success()
        .stdout(predicates::str::contains("_rustrs"));
}

#[test]
fn docs_unsupported_exits_two() {
    let (_dir, root) = scaffold_minimal();
    rustrs()
        .current_dir(&root)
        .args(["docs"])
        .assert()
        .failure()
        .code(2);
}

#[test]
fn audit_missing_tool_exits_four() {
    let (_dir, root) = scaffold_minimal();
    let mut cmd = rustrs();
    cmd.current_dir(&root).arg("audit").env("PATH", "");
    cmd.assert().failure().code(4);
}

#[test]
fn generate_docker_creates_assets() {
    let (_dir, root) = scaffold_minimal();
    rustrs()
        .current_dir(&root)
        .args(["generate", "docker"])
        .assert()
        .success();
    assert!(root.join("Dockerfile").is_file());
    assert!(root.join(".dockerignore").is_file());
}

#[test]
fn generate_ci_github_creates_workflow() {
    let (_dir, root) = scaffold_minimal();
    rustrs()
        .current_dir(&root)
        .args(["generate", "ci-github"])
        .assert()
        .success();
    assert!(root.join(".github/workflows/ci.yml").is_file());
    let contents = fs::read_to_string(root.join(".github/workflows/ci.yml")).unwrap();
    assert!(contents.contains("cargo check"));
}

#[test]
fn generate_docker_dry_run_does_not_create_files() {
    let (_dir, root) = scaffold_minimal();
    rustrs()
        .current_dir(&root)
        .args(["generate", "docker", "--dry-run"])
        .assert()
        .success();
    assert!(!root.join("Dockerfile").is_file());
}
