use std::path::{Component, Path, PathBuf};

pub fn allowed(directory: &Path, path: &str) -> Result<(), String> {
    if path.trim().is_empty() {
        return Err("READ requires a file path".into());
    }
    let root = directory
        .canonicalize()
        .map_err(|error| format!("READ cannot resolve directory: {error}"))?;
    let requested = PathBuf::from(path);
    let components: Vec<_> = requested.components().collect();
    if components.is_empty()
        || !components
            .iter()
            .all(|component| matches!(component, Component::Normal(_)))
    {
        return Err("READ path must be inside the execution directory".into());
    }
    if super::ignored::read_ignored(&root, &root.join(path))? {
        return Err("AccessDenied".into());
    }
    Ok(())
}
