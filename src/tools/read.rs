//! READ: return the UTF-8 contents of a file inside the execution directory.
use std::{fs, path::Path};

pub fn read(directory: &Path, path: &str) -> Result<String, String> {
    if path.trim().is_empty() {
        return Err("READ requires a file path".into());
    }
    let root = directory
        .canonicalize()
        .map_err(|e| format!("READ cannot resolve directory: {e}"))?;
    let file = root
        .join(path)
        .canonicalize()
        .map_err(|e| format!("READ cannot resolve `{path}`: {e}"))?;
    if !file.starts_with(&root) {
        return Err("READ path must be inside the execution directory".into());
    }
    if !file.is_file() {
        return Err(format!("READ path is not a regular file: {path}"));
    }
    fs::read_to_string(&file).map_err(|e| format!("READ cannot read `{path}` as UTF-8: {e}"))
}
