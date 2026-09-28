use super::run::run;
use std::path::{Path, PathBuf};

pub(super) struct Project {
    pub(super) directory: PathBuf,
    pub(super) root: PathBuf,
    pub(super) prefix: String,
}

pub(super) fn project(directory: &Path) -> Result<Project, String> {
    let directory = directory
        .canonicalize()
        .map_err(|error| format!("could not resolve `{}`: {error}", directory.display()))?;
    if !directory.is_dir() {
        return Err(format!(
            "Git directory is not a directory: {}",
            directory.display()
        ));
    }
    let root = String::from_utf8(run(&directory, &["rev-parse", "--show-toplevel"])?)
        .map_err(|error| format!("Git project path is not UTF-8: {error}"))?;
    let prefix = String::from_utf8(run(&directory, &["rev-parse", "--show-prefix"])?)
        .map_err(|error| format!("Git project path is not UTF-8: {error}"))?;
    Ok(Project {
        directory,
        root: PathBuf::from(root.trim_end_matches(['\r', '\n'])),
        prefix: prefix.trim_end_matches(['\r', '\n']).to_owned(),
    })
}
