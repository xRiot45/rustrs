use crate::process::spec::{ProcessError, ProcessResult, ProcessSpec};
use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub trait ProcessRunner: Send + Sync {
    fn run(&self, spec: ProcessSpec) -> Result<ProcessResult, ProcessError>;
    fn spawn(&self, spec: ProcessSpec) -> Result<std::process::Child, ProcessError>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SystemProcessRunner;

impl ProcessRunner for SystemProcessRunner {
    fn run(&self, spec: ProcessSpec) -> Result<ProcessResult, ProcessError> {
        let program = resolve_program(&spec.program)?;
        let mut command = Command::new(program);
        command.args(&spec.args);
        if let Some(cwd) = &spec.cwd {
            command.current_dir(cwd);
        }
        for (key, value) in &spec.env {
            command.env(key, value);
        }
        if spec.stdin_null {
            command.stdin(Stdio::null());
        }
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let mut child = command
            .spawn()
            .map_err(|error| ProcessError::Spawn(error.to_string()))?;

        let mut stdout = String::new();
        let mut stderr = String::new();
        if let Some(mut out) = child.stdout.take() {
            let _ = out.read_to_string(&mut stdout);
        }
        if let Some(mut err) = child.stderr.take() {
            let _ = err.read_to_string(&mut stderr);
        }
        let status = child
            .wait()
            .map_err(|error| ProcessError::Io(error.to_string()))?;

        Ok(ProcessResult {
            code: status.code().unwrap_or(-1),
            stdout,
            stderr,
        })
    }

    fn spawn(&self, spec: ProcessSpec) -> Result<std::process::Child, ProcessError> {
        let program = resolve_program(&spec.program)?;
        let mut command = Command::new(program);
        command.args(&spec.args);
        if let Some(cwd) = &spec.cwd {
            command.current_dir(cwd);
        }
        for (key, value) in &spec.env {
            command.env(key, value);
        }
        if spec.stdin_null {
            command.stdin(Stdio::null());
        }
        command.stdout(Stdio::inherit());
        command.stderr(Stdio::inherit());

        command
            .spawn()
            .map_err(|error| ProcessError::Spawn(error.to_string()))
    }
}

fn resolve_program(program: &str) -> Result<PathBuf, ProcessError> {
    if let Some(path) = find_executable(program) {
        Ok(path)
    } else {
        Err(ProcessError::NotFound(program.to_string()))
    }
}

pub fn find_executable(name: &str) -> Option<PathBuf> {
    crate::process::tools::find_executable(name)
}
