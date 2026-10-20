use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn rustrs() -> Command {
    Command::cargo_bin("rustrs").unwrap()
}

#[test]
fn generate_help_lists_supported_generators() {
    rustrs()
        .args(["generate", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("module"))
        .stdout(predicate::str::contains("resource"));
}

#[test]
fn generate_alias_help_works() {
    rustrs().args(["g", "--help"]).assert().success();
}

#[test]
fn generate_unknown_type_fails() {
    rustrs()
        .args(["generate", "definitely-unknown", "foo"])
        .assert()
        .failure()
        .code(2);
}

#[test]
fn generate_missing_args_fails() {
    rustrs().args(["generate"]).assert().failure().code(2);
}

#[test]
fn generate_outside_project_fails() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["generate", "module", "products"])
        .assert()
        .failure();
}

#[test]
fn generate_module_dry_run_does_not_create_files() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--defaults"])
        .assert()
        .success();

    let root = dir.path().join("demo");
    let before: Vec<_> = walkdir_files(&root);

    rustrs()
        .current_dir(&root)
        .args(["generate", "module", "products", "--dry-run"])
        .assert()
        .success();

    let after: Vec<_> = walkdir_files(&root);
    assert_eq!(before, after);
    assert!(!root.join("src/products.rs").exists());
}

#[test]
fn generate_module_creates_file_and_registration() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--defaults"])
        .assert()
        .success();

    let root = dir.path().join("demo");
    rustrs()
        .current_dir(&root)
        .args(["generate", "module", "products"])
        .assert()
        .success();

    assert!(root.join("src/products.rs").is_file());
    let main_rs = fs::read_to_string(root.join("src/main.rs")).unwrap();
    assert!(main_rs.contains("mod products;"));
}

#[test]
fn generate_resource_creates_expected_files() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--template", "standard", "--defaults"])
        .assert()
        .success();

    let root = dir.path().join("demo");
    rustrs()
        .current_dir(&root)
        .args([
            "generate",
            "resource",
            "products",
            "--singular",
            "product",
            "--plural",
            "products",
        ])
        .assert()
        .success();

    assert!(root.join("src/modules/products/mod.rs").is_file());
    assert!(root.join("src/modules/products/entity.rs").is_file());
    assert!(root.join("src/modules/products/controller.rs").is_file());
    assert!(root.join("src/modules/products/service.rs").is_file());
}

#[test]
fn generate_force_overwrites_existing_file() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--defaults"])
        .assert()
        .success();

    let root = dir.path().join("demo");
    fs::write(root.join("src/products.rs"), "old content").unwrap();

    rustrs()
        .current_dir(&root)
        .args(["generate", "module", "products", "--force"])
        .assert()
        .success();

    let content = fs::read_to_string(root.join("src/products.rs")).unwrap();
    assert!(content.contains("products module") || content.contains("products"));
}

#[test]
fn generate_entity_creates_file() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--template", "standard", "--defaults"])
        .assert()
        .success();
    let root = dir.path().join("demo");
    rustrs()
        .current_dir(&root)
        .args(["generate", "module", "products"])
        .assert()
        .success();
    rustrs()
        .current_dir(&root)
        .args(["generate", "entity", "products"])
        .assert()
        .success();
    assert!(root.join("src/modules/products/entity.rs").is_file());
}

#[test]
fn generate_dto_creates_file() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--template", "standard", "--defaults"])
        .assert()
        .success();
    let root = dir.path().join("demo");
    rustrs()
        .current_dir(&root)
        .args(["generate", "module", "products"])
        .assert()
        .success();
    rustrs()
        .current_dir(&root)
        .args(["generate", "dto", "products"])
        .assert()
        .success();
    assert!(root.join("src/modules/products/dto.rs").is_file());
}

#[test]
fn generate_service_creates_file() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--template", "standard", "--defaults"])
        .assert()
        .success();
    let root = dir.path().join("demo");
    rustrs()
        .current_dir(&root)
        .args(["generate", "module", "products"])
        .assert()
        .success();
    rustrs()
        .current_dir(&root)
        .args(["generate", "service", "products"])
        .assert()
        .success();
    assert!(root.join("src/modules/products/service.rs").is_file());
}

#[test]
fn generate_repository_creates_file() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--template", "standard", "--defaults"])
        .assert()
        .success();
    let root = dir.path().join("demo");
    rustrs()
        .current_dir(&root)
        .args(["generate", "module", "products"])
        .assert()
        .success();
    rustrs()
        .current_dir(&root)
        .args(["generate", "repository", "products"])
        .assert()
        .success();
    assert!(root.join("src/modules/products/repository.rs").is_file());
}

#[test]
fn generate_controller_creates_file() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--template", "standard", "--defaults"])
        .assert()
        .success();
    let root = dir.path().join("demo");
    rustrs()
        .current_dir(&root)
        .args(["generate", "module", "products"])
        .assert()
        .success();
    rustrs()
        .current_dir(&root)
        .args(["generate", "controller", "products"])
        .assert()
        .success();
    assert!(root.join("src/modules/products/controller.rs").is_file());
}

#[test]
fn generate_middleware_creates_file() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--template", "standard", "--defaults"])
        .assert()
        .success();
    let root = dir.path().join("demo");
    rustrs()
        .current_dir(&root)
        .args(["generate", "module", "products"])
        .assert()
        .success();
    rustrs()
        .current_dir(&root)
        .args(["generate", "middleware", "products"])
        .assert()
        .success();
    assert!(root.join("src/modules/products/middleware.rs").is_file());
}

#[test]
fn generate_test_creates_file() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--template", "standard", "--defaults"])
        .assert()
        .success();
    let root = dir.path().join("demo");
    rustrs()
        .current_dir(&root)
        .args(["generate", "module", "products"])
        .assert()
        .success();
    rustrs()
        .current_dir(&root)
        .args(["generate", "test", "products"])
        .assert()
        .success();
    assert!(root.join("src/modules/products/tests.rs").is_file());
}

#[test]
fn generate_migration_creates_file() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--template", "standard", "--defaults"])
        .assert()
        .success();
    let root = dir.path().join("demo");
    rustrs()
        .current_dir(&root)
        .args(["generate", "migration", "add_status"])
        .assert()
        .success();
    let entries: Vec<_> = std::fs::read_dir(root.join("migrations"))
        .unwrap()
        .collect();
    assert!(!entries.is_empty());
}

#[test]
fn generate_guard_fails_with_prerequisite_message() {
    let dir = TempDir::new().unwrap();
    rustrs()
        .current_dir(dir.path())
        .args(["new", "demo", "--template", "standard", "--defaults"])
        .assert()
        .success();
    let root = dir.path().join("demo");
    rustrs()
        .current_dir(&root)
        .args(["generate", "guard", "products"])
        .assert()
        .failure();
}

fn walkdir_files(root: &std::path::Path) -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(walkdir_files(&path));
            } else {
                files.push(path.strip_prefix(root).unwrap().display().to_string());
            }
        }
    }
    files.sort();
    files
}
