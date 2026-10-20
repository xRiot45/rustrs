use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn rustrs() -> Command {
    Command::cargo_bin("rustrs").unwrap()
}

fn scaffold(template: &str) -> (TempDir, std::path::PathBuf) {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--template", template, "--defaults"])
        .assert()
        .success();
    let root = dir.path().join("demo");
    (dir, root)
}

#[test]
fn info_help_works() {
    rustrs().args(["info", "--help"]).assert().success();
}

#[test]
fn modules_help_works() {
    rustrs().args(["modules", "--help"]).assert().success();
}

#[test]
fn routes_help_works() {
    rustrs().args(["routes", "--help"]).assert().success();
}

#[test]
fn deps_help_works() {
    rustrs().args(["deps", "--help"]).assert().success();
}

#[test]
fn doctor_help_works() {
    rustrs().args(["doctor", "--help"]).assert().success();
}

#[test]
fn info_runs_on_valid_project() {
    let (_dir, root) = scaffold("minimal");
    rustrs()
        .current_dir(&root)
        .args(["info"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Project: demo"));
}

#[test]
fn info_json_is_valid() {
    let (_dir, root) = scaffold("minimal");
    let output = rustrs()
        .current_dir(&root)
        .args(["info", "--json"])
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
fn modules_runs_on_valid_project() {
    let (_dir, root) = scaffold("standard");
    rustrs()
        .current_dir(&root)
        .args(["modules"])
        .assert()
        .success();
}

#[test]
fn routes_runs_on_valid_project() {
    let (_dir, root) = scaffold("minimal");
    rustrs()
        .current_dir(&root)
        .args(["routes"])
        .assert()
        .success()
        .stdout(predicate::str::contains("GET"));
}

#[test]
fn deps_runs_on_valid_project() {
    let (_dir, root) = scaffold("minimal");
    rustrs()
        .current_dir(&root)
        .args(["deps"])
        .assert()
        .success()
        .stdout(predicate::str::contains("axum"));
}

#[test]
fn doctor_succeeds_on_valid_project() {
    let (_dir, root) = scaffold("minimal");
    rustrs()
        .current_dir(&root)
        .args(["doctor"])
        .assert()
        .success();
}

#[test]
fn doctor_fails_on_missing_manifest() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["doctor"])
        .assert()
        .failure()
        .code(1);
}

#[test]
fn info_outside_project_fails() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["info"])
        .assert()
        .failure();
}

#[test]
fn inspection_commands_are_read_only() {
    let (_dir, root) = scaffold("minimal");
    // snapshot all files
    fn snapshot(root: &std::path::Path) -> Vec<(String, Vec<u8>)> {
        let mut files = Vec::new();
        fn walk(root: &std::path::Path, base: &std::path::Path, out: &mut Vec<(String, Vec<u8>)>) {
            if let Ok(entries) = fs::read_dir(base) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        walk(root, &path, out);
                    } else if path.is_file() {
                        let rel = path.strip_prefix(root).unwrap().display().to_string();
                        let data = fs::read(&path).unwrap();
                        out.push((rel, data));
                    }
                }
            }
        }
        walk(root, root, &mut files);
        files.sort();
        files
    }

    let before = snapshot(&root);
    for cmd in ["info", "modules", "routes", "deps", "doctor"] {
        let _ = rustrs().current_dir(&root).arg(cmd).assert();
    }
    let after = snapshot(&root);
    assert_eq!(before, after);
}

#[test]
fn doctor_reports_missing_required_file() {
    let (_dir, root) = scaffold("minimal");
    fs::remove_file(root.join("README.md")).unwrap();
    rustrs()
        .current_dir(&root)
        .args(["doctor"])
        .assert()
        .failure()
        .code(1)
        .stdout(predicate::str::contains("MISSING_REQUIRED_FILE"));
}

#[test]
fn deps_json_is_valid() {
    let (_dir, root) = scaffold("minimal");
    let output = rustrs()
        .current_dir(&root)
        .args(["deps", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    let value: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert!(value["dependencies"].is_array());
}

#[test]
fn routes_json_is_valid() {
    let (_dir, root) = scaffold("minimal");
    let output = rustrs()
        .current_dir(&root)
        .args(["routes", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    let value: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert!(value["routes"].is_array());
}

#[test]
fn modules_json_is_valid() {
    let (_dir, root) = scaffold("standard");
    let output = rustrs()
        .current_dir(&root)
        .args(["modules", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    let value: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert!(value["modules"].is_array());
}

#[test]
fn doctor_json_is_valid_on_error() {
    let dir = TempDir::new().unwrap();
    let output = rustrs()
        .current_dir(dir.path())
        .args(["doctor", "--json"])
        .assert()
        .failure()
        .code(1)
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(output).unwrap();
    let value: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert!(value["findings"].is_array());
}
