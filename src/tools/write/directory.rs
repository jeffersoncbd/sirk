use std::{fs, path::Path};

pub(super) fn ensure_directory(directory: &Path, path: &str) -> Result<(), String> {
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
