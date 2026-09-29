use std::{
    fs,
    path::{Component, Path, PathBuf},
};

pub(super) fn target(directory: &Path, path: &str, allow_missing: bool) -> Result<PathBuf, String> {
    let root = directory.canonicalize().map_err(|e| e.to_string())?;
    let components: Vec<_> = Path::new(path).components().collect();
    if components.is_empty()
        || !components
            .iter()
            .all(|component| matches!(component, Component::Normal(_)))
    {
        return Err("EDIT path must stay inside the execution directory".into());
    }
    let mut requested = root.clone();
    let mut missing_parent = false;
    for component in &components[..components.len() - 1] {
        let Component::Normal(name) = component else {
            unreachable!("validated normal path component");
        };
        requested.push(name);
        if missing_parent {
            continue;
        }
        match fs::symlink_metadata(&requested) {
            Ok(metadata) if metadata.file_type().is_dir() => {}
            Ok(_) => return Err("EDIT cannot traverse a non-directory path component".into()),
            Err(error) if allow_missing && error.kind() == std::io::ErrorKind::NotFound => {
                missing_parent = true;
            }
            Err(error) => return Err(format!("EDIT cannot inspect path: {error}")),
        }
    }
    let Component::Normal(name) = components.last().unwrap() else {
        unreachable!("validated normal path component");
    };
    requested.push(name);
    if missing_parent {
        return Ok(requested);
    }
    let metadata = match fs::symlink_metadata(&requested) {
        Ok(metadata) => metadata,
        Err(error) if allow_missing && error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(requested);
        }
        Err(error) => return Err(format!("EDIT cannot inspect path: {error}")),
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
