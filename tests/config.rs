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
fn config_help_works() {
    rustrs().args(["config", "--help"]).assert().success();
}

#[test]
fn config_show_runs() {
    let (_dir, root) = scaffold();
    rustrs()
        .current_dir(&root)
        .args(["config", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("template = \"minimal\""));
}

#[test]
fn config_show_json_is_valid() {
    let (_dir, root) = scaffold();
    let output = rustrs()
        .current_dir(&root)
        .args(["config", "show", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    let value: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(value["project_name"], "demo");
}

#[test]
fn config_get_runs() {
    let (_dir, root) = scaffold();
    rustrs()
        .current_dir(&root)
        .args(["config", "get", "features.auth"])
        .assert()
        .success()
        .stdout(predicate::str::contains("false"));
}

#[test]
fn config_set_updates_manifest() {
    let (_dir, root) = scaffold();
    rustrs()
        .current_dir(&root)
        .args(["config", "set", "features.auth", "true"])
        .assert()
        .success();
    let contents = fs::read_to_string(root.join(".rustrs/config.toml")).unwrap();
    assert!(contents.contains("auth = true"));
}

#[test]
fn config_set_invalid_key_fails() {
    let (_dir, root) = scaffold();
    let before = fs::read_to_string(root.join(".rustrs/config.toml")).unwrap();
    rustrs()
        .current_dir(&root)
        .args(["config", "set", "nope", "true"])
        .assert()
        .failure()
        .code(2);
    let after = fs::read_to_string(root.join(".rustrs/config.toml")).unwrap();
    assert_eq!(before, after);
}

#[test]
fn config_set_invalid_value_fails() {
    let (_dir, root) = scaffold();
    let before = fs::read_to_string(root.join(".rustrs/config.toml")).unwrap();
    rustrs()
        .current_dir(&root)
        .args(["config", "set", "features.auth", "maybe"])
        .assert()
        .failure()
        .code(2);
    let after = fs::read_to_string(root.join(".rustrs/config.toml")).unwrap();
    assert_eq!(before, after);
}

#[test]
fn config_set_unsupported_combination_fails() {
    let (_dir, root) = scaffold();
    let before = fs::read_to_string(root.join(".rustrs/config.toml")).unwrap();
    rustrs()
        .current_dir(&root)
        .args(["config", "set", "database", "none"])
        .assert()
        .failure()
        .code(2);
    let after = fs::read_to_string(root.join(".rustrs/config.toml")).unwrap();
    assert_eq!(before, after);
}

#[test]
fn config_outside_project_fails() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["config", "show"])
        .assert()
        .failure();
}

#[test]
fn config_set_does_not_change_other_files() {
    let (_dir, root) = scaffold();
    let before = fs::read_to_string(root.join("Cargo.toml")).unwrap();
    rustrs()
        .current_dir(&root)
        .args(["config", "set", "features.auth", "true"])
        .assert()
        .success();
    let after = fs::read_to_string(root.join("Cargo.toml")).unwrap();
    assert_eq!(before, after);
}
