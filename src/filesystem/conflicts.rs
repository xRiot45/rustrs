use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictStatus {
    Absent,
    ExistsFile,
    ExistsDirectory,
    Invalid,
}

pub fn detect_conflict(root: &Path, relative_path: &Path) -> ConflictStatus {
    let candidate = root.join(relative_path);
    if !candidate.exists() {
        return ConflictStatus::Absent;
    }
    if candidate.is_file() {
        ConflictStatus::ExistsFile
    } else if candidate.is_dir() {
        ConflictStatus::ExistsDirectory
    } else {
        ConflictStatus::Invalid
    }
}
