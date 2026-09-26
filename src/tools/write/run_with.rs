use super::directory::ensure_directory;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Component, Path},
};

#[cfg(test)]
mod tests;

pub fn write_with_options(
    directory: &Path,
    path: &str,
    content: &str,
    force: bool,
    skip: bool,
) -> Result<(), String> {
    if force && skip {
        return Err("WRITE cannot combine force: true with skip: true".into());
    }
    if path.trim().is_empty() {
        return Err("WRITE requires a path".into());
    }
    let root = directory
        .canonicalize()
        .map_err(|error| format!("WRITE cannot resolve directory: {error}"))?;
    let requested = Path::new(path);
    let requested = if requested.is_absolute() {
        requested.to_owned()
    } else {
        root.join(requested)
    };
    let relative = requested
        .strip_prefix(&root)
        .map_err(|_| "WRITE path must be inside the execution directory")?;
    let mut components = relative.components().peekable();
    let mut target = root.clone();
    while let Some(component) = components.next() {
        let Component::Normal(name) = component else {
            return Err("WRITE path must be inside the execution directory".into());
        };
        target.push(name);
        if components.peek().is_some() {
            ensure_directory(&target, path)?;
        }
    }
    match fs::symlink_metadata(&target) {
        Ok(metadata) => {
            if !metadata.file_type().is_file() {
                return Err(format!("WRITE path is not a regular file: {path}"));
            }
            let resolved = target
                .canonicalize()
                .map_err(|error| format!("WRITE cannot resolve `{path}`: {error}"))?;
            if !resolved.starts_with(&root) {
                return Err("WRITE path must be inside the execution directory".into());
            }
            if skip {
                return Ok(());
            }
            if !force {
                return Err(format!(
                    "WRITE refuses to overwrite existing file `{path}`; set force: true to replace it"
                ));
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => (),
        Err(error) => return Err(format!("WRITE cannot inspect `{path}`: {error}")),
    }
    let mut options = OpenOptions::new();
    options.write(true);
    if force {
        options.create(true).truncate(true);
    } else {
        options.create_new(true);
    }
    let mut file = match options.open(&target) {
        Ok(file) => file,
        Err(error) if skip && error.kind() == std::io::ErrorKind::AlreadyExists => {
            // A concurrent creator won; only regular files qualify for skipping.
            let metadata = fs::symlink_metadata(&target)
                .map_err(|e| format!("WRITE cannot inspect `{path}`: {e}"))?;
            if !metadata.file_type().is_file() {
                return Err(format!("WRITE path is not a regular file: {path}"));
            }
            return Ok(());
        }
        Err(error) => return Err(format!("WRITE cannot create `{path}`: {error}")),
    };
    file.write_all(content.as_bytes())
        .and_then(|_| file.sync_all())
        .map_err(|error| format!("WRITE cannot write `{path}`: {error}"))
}
