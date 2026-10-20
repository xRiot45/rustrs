use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ProcessSpec {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: Option<PathBuf>,
    pub env: Vec<(String, String)>,
    pub stdin_null: bool,
}

impl ProcessSpec {
    pub fn new(program: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
            cwd: None,
            env: Vec::new(),
            stdin_null: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProcessResult {
    pub code: i32,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ProcessError {
    #[error("executable not found: {0}")]
    NotFound(String),
    #[error("executable is not runnable: {0}")]
    NotRunnable(String),
    #[error("failed to spawn process: {0}")]
    Spawn(String),
    #[error("io error: {0}")]
    Io(String),
    #[error("process cancelled")]
    Cancelled,
}
