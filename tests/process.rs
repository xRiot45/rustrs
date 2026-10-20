use rustrs::process::compose::detect_compose;
use rustrs::process::tools::find_executable;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use tempfile::TempDir;

#[test]
fn detect_compose_without_file() {
    let dir = TempDir::new().unwrap();
    let detection = detect_compose(dir.path());
    assert!(detection.compose_file.is_none());
}

#[test]
fn detect_compose_with_file() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("compose.yml"), "services: {}\n").unwrap();
    let detection = detect_compose(dir.path());
    assert!(detection.compose_file.is_some());
}

#[cfg(unix)]
#[test]
fn find_executable_rejects_non_executable() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("fake-tool");
    fs::write(&path, "#!/bin/sh\necho ok\n").unwrap();
    let mut perms = fs::metadata(&path).unwrap().permissions();
    perms.set_mode(0o644);
    fs::set_permissions(&path, perms).unwrap();
    assert!(find_executable(path.to_str().unwrap()).is_none());

    let mut perms = fs::metadata(&path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&path, perms).unwrap();
    assert!(find_executable(path.to_str().unwrap()).is_some());
}
