use assert_cmd::Command;
use std::process::Command as StdCommand;
use tempfile::TempDir;

fn rustrs() -> Command {
    Command::cargo_bin("rustrs").unwrap()
}

#[test]
#[ignore = "requires network/cache and builds generated project dependencies"]
fn generated_minimal_project_passes_cargo_check() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--template", "minimal", "--defaults"])
        .assert()
        .success();
    let root = dir.path().join("demo");
    let status = StdCommand::new("cargo")
        .arg("check")
        .current_dir(&root)
        .status()
        .unwrap();
    assert!(status.success());
}

#[test]
#[ignore = "requires network/cache and builds generated project dependencies"]
fn generated_minimal_project_passes_cargo_fmt_check() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--template", "minimal", "--defaults"])
        .assert()
        .success();
    let root = dir.path().join("demo");
    let status = StdCommand::new("cargo")
        .args(["fmt", "--check"])
        .current_dir(&root)
        .status()
        .unwrap();
    assert!(status.success());
}
