//! DELETE: remove a regular file within the execution directory.
use std::{
    fs::{self, File},
    path::{Component, Path},
};

pub fn delete(directory: &Path, path: &str) -> Result<(), String> {
    if path.trim().is_empty() {
        return Err("DELETE requires a path".into());
    }
    let root = directory
        .canonicalize()
        .map_err(|error| format!("DELETE cannot resolve directory: {error}"))?;
    let requested = Path::new(path);
    let requested = if requested.is_absolute() {
        requested.to_owned()
    } else {
        root.join(requested)
    };
    let relative = requested
        .strip_prefix(&root)
        .map_err(|_| "DELETE path must be inside the execution directory")?;
    let components: Vec<_> = relative.components().collect();
    if components.is_empty()
        || !components
            .iter()
            .all(|component| matches!(component, Component::Normal(_)))
    {
        return Err("DELETE path must be inside the execution directory".into());
    }
    let mut target = root.clone();
    for component in &components {
        let Component::Normal(name) = component else {
            unreachable!("validated normal path component");
        };
        target.push(name);
    }
    let mut parent = root.clone();
    for component in &components[..components.len() - 1] {
        let Component::Normal(name) = component else {
            unreachable!("validated normal path component");
        };
        parent.push(name);
        let metadata = fs::symlink_metadata(&parent)
            .map_err(|error| format!("DELETE cannot inspect `{path}`: {error}"))?;
        if !metadata.file_type().is_dir() {
            return Err(format!(
                "DELETE cannot traverse non-directory component `{}`",
                name.to_string_lossy()
            ));
        }
    }
    let metadata = fs::symlink_metadata(&target)
        .map_err(|error| format!("DELETE cannot inspect `{path}`: {error}"))?;
    if !metadata.file_type().is_file() {
        return Err(format!("DELETE path is not a regular file: {path}"));
    }
    let resolved = target
        .canonicalize()
        .map_err(|error| format!("DELETE cannot resolve `{path}`: {error}"))?;
    if !resolved.starts_with(&root) {
        return Err("DELETE path must be inside the execution directory".into());
    }
    fs::remove_file(&target).map_err(|error| format!("DELETE cannot remove `{path}`: {error}"))?;
    File::open(target.parent().expect("target has parent"))
        .and_then(|directory| directory.sync_all())
        .map_err(|error| {
            format!("DELETE cannot synchronize parent directory for `{path}`: {error}")
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_project() -> std::path::PathBuf {
        let directory = std::env::temp_dir().join(format!(
            "delete-tool-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&directory).unwrap();
        directory
    }

    #[test]
    fn removes_regular_files_inside_the_project() {
        let project = temporary_project();
        fs::create_dir(project.join("nested")).unwrap();
        fs::write(project.join("nested/note.txt"), "obsolete").unwrap();
        delete(&project, "nested/note.txt").unwrap();
        assert!(!project.join("nested/note.txt").exists());
        fs::remove_dir_all(project).unwrap();
    }

    #[test]
    fn rejects_outside_paths_directories_and_links() {
        let project = temporary_project();
        fs::create_dir(project.join("nested")).unwrap();
        fs::write(project.join("file"), "content").unwrap();
        assert!(delete(&project, "../outside").is_err());
        assert!(delete(&project, "nested").is_err());
        assert!(delete(&project, "missing").is_err());
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(project.join("file"), project.join("link")).unwrap();
            std::os::unix::fs::symlink(project.join("nested"), project.join("linked-dir")).unwrap();
            assert!(delete(&project, "link").is_err());
            assert!(delete(&project, "linked-dir/file").is_err());
        }
        fs::remove_dir_all(project).unwrap();
    }
}
