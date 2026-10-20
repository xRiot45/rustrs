use rustrs::compatibility::CompatibilityError;
use rustrs::error::CliError;
use rustrs::manifest::ManifestError;
use rustrs::project::ProjectRootError;
use std::io;

#[test]
fn invalid_input_maps_to_exit_code_two() {
    assert_eq!(CliError::InvalidInput("bad".to_string()).exit_code(), 2);
}

#[test]
fn project_root_maps_to_exit_code_two() {
    let err = CliError::ProjectRoot(ProjectRootError::NotFound {
        start: std::path::PathBuf::from("missing"),
    });
    assert_eq!(err.exit_code(), 2);
}

#[test]
fn manifest_io_maps_to_exit_code_one() {
    let err = CliError::Manifest(ManifestError::Io {
        path: std::path::PathBuf::from("missing.toml"),
        source: io::Error::new(io::ErrorKind::NotFound, "not found"),
    });
    assert_eq!(err.exit_code(), 1);
}

#[test]
fn manifest_config_errors_map_to_exit_code_two() {
    let err = CliError::Manifest(ManifestError::Parse {
        message: "bad".to_string(),
    });
    assert_eq!(err.exit_code(), 2);

    let err = CliError::Manifest(ManifestError::UnsupportedSchemaVersion { version: 2 });
    assert_eq!(err.exit_code(), 2);

    let err = CliError::Manifest(ManifestError::Validation {
        message: "bad".to_string(),
    });
    assert_eq!(err.exit_code(), 2);
}

#[test]
fn compatibility_error_maps_to_exit_code_two() {
    let err = CliError::Compatibility(CompatibilityError::Unsupported {
        reason: "unsupported".to_string(),
        alternatives: vec!["axum".to_string()],
    });
    assert_eq!(err.exit_code(), 2);
}
