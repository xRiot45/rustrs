use crate::process::spec::ProcessError;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Serialize)]
pub struct ToolStatus {
    pub name: String,
    pub available: bool,
    pub path: Option<PathBuf>,
    pub version: Option<String>,
}

fn is_executable(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = path.metadata() {
            return metadata.permissions().mode() & 0o111 != 0;
        }
        false
    }

    #[cfg(windows)]
    {
        let extensions =
            std::env::var("PATHEXT").unwrap_or_else(|_| ".EXE;.BAT;.CMD;.COM".to_string());
        let path_ext = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| format!(".{}", ext.to_ascii_uppercase()))
            .unwrap_or_default();
        if extensions
            .split(';')
            .any(|ext| ext.eq_ignore_ascii_case(&path_ext))
        {
            return true;
        }
        return path_ext == ".EXE";
    }

    #[cfg(not(any(unix, windows)))]
    {
        true
    }
}

pub fn find_executable(name: &str) -> Option<PathBuf> {
    if name.contains(std::path::MAIN_SEPARATOR) || name.contains('/') || name.contains('\\') {
        let path = PathBuf::from(name);
        if is_executable(&path) {
            return Some(path);
        }
        return None;
    }

    let path_var = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path_var) {
        let candidate = dir.join(name);
        if is_executable(&candidate) {
            return Some(candidate);
        }

        #[cfg(windows)]
        {
            let extensions =
                std::env::var("PATHEXT").unwrap_or_else(|_| ".EXE;.BAT;.CMD;.COM".to_string());
            for ext in extensions.split(';') {
                if ext.is_empty() {
                    continue;
                }
                let candidate = dir.join(format!("{name}{}", ext));
                if is_executable(&candidate) {
                    return Some(candidate);
                }
            }
        }
    }
    None
}

pub fn check_tool(name: &str) -> ToolStatus {
    if let Some(path) = find_executable(name) {
        ToolStatus {
            name: name.to_string(),
            available: true,
            path: Some(path.clone()),
            version: tool_version(&path),
        }
    } else {
        ToolStatus {
            name: name.to_string(),
            available: false,
            path: None,
            version: None,
        }
    }
}

pub fn tool_version(path: &Path) -> Option<String> {
    let output = Command::new(path).arg("--version").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let line = text.lines().next().unwrap_or("").trim();
    if line.is_empty() {
        None
    } else {
        Some(line.to_string())
    }
}

pub fn require_tool(name: &str) -> Result<PathBuf, ProcessError> {
    find_executable(name).ok_or_else(|| ProcessError::NotFound(name.to_string()))
}
