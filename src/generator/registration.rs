use crate::error::CliError;
use std::path::Path;

pub fn insert_between_markers(
    existing: Option<&str>,
    begin_marker: &str,
    end_marker: &str,
    line: &str,
) -> Result<String, CliError> {
    let content = existing.unwrap_or("");
    if content.contains(line) {
        return Ok(content.to_string());
    }

    let begin_idx = content
        .find(begin_marker)
        .ok_or_else(|| CliError::InvalidInput(format!("missing marker: {begin_marker}")))?;
    let end_idx = content
        .find(end_marker)
        .ok_or_else(|| CliError::InvalidInput(format!("missing marker: {end_marker}")))?;
    if end_idx < begin_idx {
        return Err(CliError::InvalidInput("invalid marker order".to_string()));
    }

    let mut result = String::with_capacity(content.len() + line.len() + 2);
    result.push_str(&content[..begin_idx + begin_marker.len()]);
    if !result.ends_with('\n') {
        result.push('\n');
    }
    result.push_str(line);
    result.push('\n');
    result.push_str(&content[begin_idx + begin_marker.len()..]);
    Ok(result)
}

pub fn ensure_markers(content: &str, begin: &str, end: &str) -> bool {
    content.contains(begin) && content.contains(end)
}

pub fn marker_pair(section: &str) -> (String, String) {
    (
        format!("// rustrs:begin:{section}"),
        format!("// rustrs:end:{section}"),
    )
}

#[allow(dead_code)]
pub fn _path_marker(_path: &Path) {}
