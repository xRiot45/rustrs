use assert_cmd::Command;
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
fn db_help_works() {
    rustrs().args(["db", "--help"]).assert().success();
}

#[test]
fn dev_help_works() {
    rustrs().args(["dev", "--help"]).assert().success();
}

#[test]
fn db_create_migration_creates_file() {
    let (_dir, root) = scaffold("standard");
    rustrs()
        .current_dir(&root)
        .args(["db", "create-migration", "add_status"])
        .assert()
        .success();
    let entries: Vec<_> = fs::read_dir(root.join("migrations")).unwrap().collect();
    assert!(!entries.is_empty());
}

#[test]
fn db_seed_is_unsupported() {
    let (_dir, root) = scaffold("standard");
    rustrs()
        .current_dir(&root)
        .args(["db", "seed"])
        .assert()
        .failure()
        .code(2);
}

#[test]
fn db_migrate_missing_tool_exits_four() {
    let (_dir, root) = scaffold("standard");
    let mut cmd = rustrs();
    cmd.current_dir(&root)
        .args(["db", "migrate"])
        .env("PATH", "");
    cmd.assert().failure().code(4);
}

#[test]
fn dev_invalid_project_exits_two() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["dev"])
        .assert()
        .failure()
        .code(2);
}

#[test]
fn dev_missing_cargo_exits_four() {
    let (_dir, root) = scaffold("minimal");
    let mut cmd = rustrs();
    cmd.current_dir(&root).arg("dev").env("PATH", "");
    cmd.assert().failure().code(4);
}
