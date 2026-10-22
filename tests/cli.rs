use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn rustrs() -> Command {
    Command::cargo_bin("rustrs").unwrap()
}

#[test]
fn help_exits_zero_and_contains_name() {
    rustrs()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("rustrs"))
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn version_exits_zero_and_matches_package() {
    rustrs()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn unknown_argument_exits_non_zero() {
    rustrs()
        .arg("--definitely-unknown")
        .assert()
        .failure()
        .code(2);
}

#[test]
fn help_works_outside_project_directory() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn version_works_outside_project_directory() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .arg("--version")
        .assert()
        .success();
}

#[test]
fn no_color_flag_is_accepted() {
    rustrs().args(["--no-color", "--help"]).assert().success();
}

#[test]
fn verbose_and_quiet_conflict() {
    rustrs()
        .args(["--verbose", "--quiet"])
        .assert()
        .failure()
        .code(2);
}

#[test]
fn basic_invocation_does_not_create_files() {
    let dir = TempDir::new().unwrap();
    let before: Vec<_> = fs::read_dir(dir.path()).unwrap().collect();
    rustrs()
        .current_dir(dir.path())
        .arg("--help")
        .assert()
        .success();
    let after: Vec<_> = fs::read_dir(dir.path()).unwrap().collect();
    assert_eq!(before.len(), after.len());
}

#[test]
fn env_check_help_works() {
    rustrs().args(["env", "check", "--help"]).assert().success();
}

#[test]
fn env_check_runs() {
    rustrs().args(["env", "check"]).assert().success();
}

#[test]
fn generate_no_interactive_help_works() {
    rustrs()
        .args(["generate", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("no-interactive"));
}

#[test]
fn generate_no_interactive_requires_name() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--defaults"])
        .assert()
        .success();
    let root = dir.path().join("demo");
    rustrs()
        .current_dir(&root)
        .args(["generate", "entity", "--no-interactive"])
        .assert()
        .failure()
        .code(2);
}

#[test]
fn new_git_flag_creates_git_directory() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--defaults", "--git"])
        .assert()
        .success();
    assert!(dir.path().join("demo/.git").is_dir());
}

#[test]
fn new_without_git_flag_does_not_create_git_directory() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--defaults"])
        .assert()
        .success();
    assert!(!dir.path().join("demo/.git").exists());
}

#[test]
fn update_check_help_works() {
    rustrs()
        .args(["update", "check", "--help"])
        .assert()
        .success();
}
