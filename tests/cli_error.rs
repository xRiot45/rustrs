use rustrs::error::CliError;
use rustrs::project::ProjectRootError;

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
