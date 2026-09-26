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
    if super::ignored::read_ignored(&root, &file)? {
        return Err("AccessDenied".into());
    }
    fs::read_to_string(&file).map_err(|e| format!("READ cannot read `{path}` as UTF-8: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn directory() -> std::path::PathBuf {
        let directory = std::env::temp_dir().join(format!(
            "read-ignore-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&directory).unwrap();
        directory
    }

    #[test]
    fn returns_access_denied_for_paths_matching_readignore() {
        let directory = directory();
        fs::create_dir(directory.join("nested")).unwrap();
        fs::write(directory.join(".env"), "secret").unwrap();
        fs::write(directory.join("nested/.env"), "secret").unwrap();
        fs::write(directory.join("nested/token.pem"), "secret").unwrap();
        fs::write(directory.join("visible.txt"), "visible").unwrap();
        fs::write(directory.join(".readignore"), ".env\n*.pem\n").unwrap();

        assert_eq!(read(&directory, ".env"), Err("AccessDenied".into()));
        assert_eq!(
            read(&directory, "nested/../nested/.env"),
            Err("AccessDenied".into())
        );
        assert_eq!(
            read(&directory, "nested/token.pem"),
            Err("AccessDenied".into())
        );
        assert_eq!(read(&directory, "visible.txt"), Ok("visible".into()));
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn blocks_a_directory_pattern_and_ignores_comments() {
        let directory = directory();
        fs::create_dir(directory.join("secrets")).unwrap();
        fs::write(directory.join("secrets/key"), "secret").unwrap();
        fs::write(directory.join("notes.txt"), "notes").unwrap();
        fs::write(
            directory.join(".readignore"),
            "# sensitive files\nsecrets/\n",
        )
        .unwrap();

        assert_eq!(read(&directory, "secrets/key"), Err("AccessDenied".into()));
        assert_eq!(read(&directory, "notes.txt"), Ok("notes".into()));
        fs::remove_dir_all(directory).unwrap();
    }
}
