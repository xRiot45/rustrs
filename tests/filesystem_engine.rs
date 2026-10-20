use rustrs::filesystem::conflicts::{ConflictStatus, detect_conflict};
use rustrs::filesystem::paths::validate_relative_path;
use rustrs::filesystem::preview_plan;
use rustrs::filesystem::writer::apply_plan;
use rustrs::generation::plan::{GenerationPlan, PlannedOperation, WritePolicy};
use std::fs;
use tempfile::TempDir;

#[test]
fn validates_relative_paths() {
    assert!(validate_relative_path(std::path::Path::new("a/b.txt")).is_ok());
    assert!(validate_relative_path(std::path::Path::new("../x")).is_err());
    assert!(validate_relative_path(std::path::Path::new("/x")).is_err());
}

#[test]
fn detects_absent_and_existing_targets() {
    let dir = TempDir::new().unwrap();
    let relative = std::path::Path::new("file.txt");

    assert_eq!(
        detect_conflict(dir.path(), relative),
        ConflictStatus::Absent
    );

    fs::write(dir.path().join("file.txt"), "x").unwrap();
    assert_eq!(
        detect_conflict(dir.path(), relative),
        ConflictStatus::ExistsFile
    );

    fs::remove_file(dir.path().join("file.txt")).unwrap();
    fs::create_dir(dir.path().join("file.txt")).unwrap();
    assert_eq!(
        detect_conflict(dir.path(), relative),
        ConflictStatus::ExistsDirectory
    );
}

#[test]
fn writer_creates_planned_files() {
    let dir = TempDir::new().unwrap();
    let plan = GenerationPlan {
        target_root: dir.path().to_path_buf(),
        operations: vec![PlannedOperation {
            relative_path: std::path::PathBuf::from("out.txt"),
            contents: b"hello".to_vec(),
            description: "create".to_string(),
        }],
    };

    let report = apply_plan(&plan, WritePolicy::PreserveExisting);
    assert!(matches!(
        report.results[0].outcome,
        rustrs::generation::plan::OperationOutcome::Created
    ));
    assert_eq!(
        fs::read_to_string(dir.path().join("out.txt")).unwrap(),
        "hello"
    );
}

#[test]
fn writer_preserves_existing_files_by_default() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("out.txt"), "original").unwrap();

    let plan = GenerationPlan {
        target_root: dir.path().to_path_buf(),
        operations: vec![PlannedOperation {
            relative_path: std::path::PathBuf::from("out.txt"),
            contents: b"new".to_vec(),
            description: "create".to_string(),
        }],
    };

    let report = apply_plan(&plan, WritePolicy::PreserveExisting);
    assert!(matches!(
        report.results[0].outcome,
        rustrs::generation::plan::OperationOutcome::Skipped { .. }
    ));
    assert_eq!(
        fs::read_to_string(dir.path().join("out.txt")).unwrap(),
        "original"
    );
}

#[test]
fn writer_creates_parent_directories() {
    let dir = TempDir::new().unwrap();
    let plan = GenerationPlan {
        target_root: dir.path().to_path_buf(),
        operations: vec![PlannedOperation {
            relative_path: std::path::PathBuf::from("nested/deep/out.txt"),
            contents: b"hello".to_vec(),
            description: "create".to_string(),
        }],
    };

    let report = apply_plan(&plan, WritePolicy::PreserveExisting);
    assert!(matches!(
        report.results[0].outcome,
        rustrs::generation::plan::OperationOutcome::Created
    ));
    assert!(dir.path().join("nested/deep/out.txt").is_file());
}

#[test]
fn preview_does_not_mutate_filesystem() {
    let dir = TempDir::new().unwrap();
    let plan = GenerationPlan {
        target_root: dir.path().to_path_buf(),
        operations: vec![PlannedOperation {
            relative_path: std::path::PathBuf::from("out.txt"),
            contents: b"hello".to_vec(),
            description: "create".to_string(),
        }],
    };

    let before: Vec<_> = fs::read_dir(dir.path()).unwrap().collect();
    let report = preview_plan(&plan);
    let after: Vec<_> = fs::read_dir(dir.path()).unwrap().collect();

    assert_eq!(before.len(), after.len());
    assert!(matches!(
        report.results[0].outcome,
        rustrs::generation::plan::OperationOutcome::Planned
    ));
}

#[test]
fn preview_marks_existing_target_as_skipped() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("out.txt"), "x").unwrap();

    let plan = GenerationPlan {
        target_root: dir.path().to_path_buf(),
        operations: vec![PlannedOperation {
            relative_path: std::path::PathBuf::from("out.txt"),
            contents: b"hello".to_vec(),
            description: "create".to_string(),
        }],
    };

    let report = preview_plan(&plan);
    assert!(matches!(
        report.results[0].outcome,
        rustrs::generation::plan::OperationOutcome::Skipped { .. }
    ));
}

#[test]
fn writer_reports_invalid_path_without_writing() {
    let dir = TempDir::new().unwrap();
    let plan = GenerationPlan {
        target_root: dir.path().to_path_buf(),
        operations: vec![PlannedOperation {
            relative_path: std::path::PathBuf::from("../escape.txt"),
            contents: b"hello".to_vec(),
            description: "create".to_string(),
        }],
    };

    let report = apply_plan(&plan, WritePolicy::PreserveExisting);
    assert!(matches!(
        report.results[0].outcome,
        rustrs::generation::plan::OperationOutcome::Failed { .. }
    ));
    assert!(!dir.path().join("../escape.txt").exists());
}

#[cfg(unix)]
#[test]
fn writer_rejects_symlink_escape() {
    use std::os::unix::fs::symlink;

    let dir = TempDir::new().unwrap();
    let outside = TempDir::new().unwrap();
    symlink(outside.path(), dir.path().join("link")).unwrap();

    let plan = GenerationPlan {
        target_root: dir.path().to_path_buf(),
        operations: vec![PlannedOperation {
            relative_path: std::path::PathBuf::from("link/escape.txt"),
            contents: b"hello".to_vec(),
            description: "create".to_string(),
        }],
    };

    let report = apply_plan(&plan, WritePolicy::PreserveExisting);
    assert!(matches!(
        report.results[0].outcome,
        rustrs::generation::plan::OperationOutcome::Failed { .. }
    ));
    assert!(!outside.path().join("escape.txt").exists());
}
