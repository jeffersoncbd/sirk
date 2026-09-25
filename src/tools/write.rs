//! WRITE: create a UTF-8 file within the execution directory.
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Component, Path},
};

pub fn write(directory: &Path, path: &str, content: &str, force: bool) -> Result<(), String> {
    write_with_options(directory, path, content, force, false)
}

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

fn ensure_directory(directory: &Path, path: &str) -> Result<(), String> {
    match fs::symlink_metadata(directory) {
        Ok(metadata) if metadata.file_type().is_dir() => Ok(()),
        Ok(_) => Err(format!(
            "WRITE cannot create parent for `{path}` because `{}` is not a directory",
            directory.display()
        )),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            match fs::create_dir(directory) {
                Ok(()) => Ok(()),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                    ensure_directory(directory, path)
                }
                Err(error) => Err(format!(
                    "WRITE cannot create parent directory `{}`: {error}",
                    directory.display()
                )),
            }
        }
        Err(error) => Err(format!(
            "WRITE cannot inspect parent directory `{}`: {error}",
            directory.display()
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_project() -> std::path::PathBuf {
        let directory = std::env::temp_dir().join(format!(
            "write-tool-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&directory).unwrap();
        directory
    }

    #[test]
    fn skip_preserves_existing_files_and_creates_missing_files() {
        let project = temporary_project();
        write_with_options(&project, "nested/file", "original\n", false, true).unwrap();
        write_with_options(&project, "nested/file", "replacement", false, true).unwrap();
        assert_eq!(
            fs::read_to_string(project.join("nested/file")).unwrap(),
            "original\n"
        );
        assert!(write_with_options(&project, "nested/file", "replacement", true, true).is_err());
        assert!(write_with_options(&project, "nested", "content", false, true).is_err());
        assert!(write_with_options(&project, "../outside", "content", false, true).is_err());
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(project.join("nested/file"), project.join("link")).unwrap();
            assert!(write_with_options(&project, "link", "content", false, true).is_err());
        }
        fs::remove_dir_all(project).unwrap();
    }

    #[test]
    fn creates_new_files_and_refuses_existing_ones_without_force() {
        let project = temporary_project();
        write(&project, "nested/deeper/note.txt", "first\n", false).unwrap();
        assert_eq!(
            fs::read_to_string(project.join("nested/deeper/note.txt")).unwrap(),
            "first\n"
        );
        let error = write(&project, "nested/deeper/note.txt", "second", false).unwrap_err();
        assert!(error.contains("force: true"));
        assert_eq!(
            fs::read_to_string(project.join("nested/deeper/note.txt")).unwrap(),
            "first\n"
        );
        fs::remove_dir_all(project).unwrap();
    }

    #[test]
    fn force_replaces_existing_file_contents() {
        let project = temporary_project();
        fs::create_dir_all(project.join("nested")).unwrap();
        fs::write(project.join("nested/note.txt"), "old content").unwrap();
        write(&project, "nested/note.txt", "new content", true).unwrap();
        assert_eq!(
            fs::read_to_string(project.join("nested/note.txt")).unwrap(),
            "new content"
        );
        fs::remove_dir_all(project).unwrap();
    }

    #[test]
    fn rejects_paths_outside_the_project_and_non_files() {
        let project = temporary_project();
        assert!(write(&project, "../outside.txt", "content", false).is_err());
        fs::create_dir(project.join("nested")).unwrap();
        assert!(write(&project, "nested", "content", true).is_err());
        write(&project, "missing/note.txt", "content", false).unwrap();
        assert_eq!(
            fs::read_to_string(project.join("missing/note.txt")).unwrap(),
            "content"
        );
        fs::remove_dir_all(project).unwrap();
    }
}
