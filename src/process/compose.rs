use crate::process::runner::{ProcessRunner, SystemProcessRunner};
use crate::process::spec::ProcessSpec;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ComposeDetection {
    pub compose_file: Option<PathBuf>,
    pub docker_available: bool,
    pub compose_available: bool,
}

const COMPOSE_FILES: [&str; 4] = [
    "docker-compose.yml",
    "docker-compose.yaml",
    "compose.yml",
    "compose.yaml",
];

pub fn detect_compose(root: &Path) -> ComposeDetection {
    let compose_file = COMPOSE_FILES
        .iter()
        .map(|name| root.join(name))
        .find(|path| path.is_file());

    let docker_available = crate::process::tools::find_executable("docker").is_some()
        || crate::process::tools::find_executable("docker-compose").is_some();

    let compose_available = if crate::process::tools::find_executable("docker").is_some() {
        let runner = SystemProcessRunner;
        let mut spec = ProcessSpec::new("docker");
        spec.args = vec!["compose".to_string(), "version".to_string()];
        spec.cwd = Some(root.to_path_buf());
        runner.run(spec).map(|r| r.code == 0).unwrap_or(false)
    } else {
        crate::process::tools::find_executable("docker-compose").is_some()
    };

    ComposeDetection {
        compose_file,
        docker_available,
        compose_available,
    }
}
