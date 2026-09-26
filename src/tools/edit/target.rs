use std::{
    fs,
    path::{Path, PathBuf},
};

pub(super) fn target(directory: &Path, path: &str, allow_missing: bool) -> Result<PathBuf, String> {
    let root = directory.canonicalize().map_err(|e| e.to_string())?;
    let requested = root.join(path);
    let metadata = match fs::symlink_metadata(&requested) {
        Ok(metadata) => metadata,
        Err(e) if allow_missing && e.kind() == std::io::ErrorKind::NotFound => {
            let parent = requested
                .parent()
                .ok_or("EDIT requires a parent directory")?
                .canonicalize()
                .map_err(|e| format!("EDIT parent directory must exist: {e}"))?;
            if !parent.starts_with(&root) {
                return Err("EDIT path must stay inside the execution directory".into());
            }
            return Ok(parent.join(requested.file_name().ok_or("EDIT requires a file name")?));
        }
        Err(e) => return Err(format!("EDIT cannot inspect path: {e}")),
    };
    if !metadata.file_type().is_file() {
        return Err("EDIT requires an existing regular file, not a symlink".into());
    }
    let resolved = requested.canonicalize().map_err(|e| e.to_string())?;
    if !resolved.starts_with(root) {
        return Err("EDIT path must stay inside the execution directory".into());
    }
    Ok(resolved)
}
