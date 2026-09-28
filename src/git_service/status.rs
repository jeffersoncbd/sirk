use super::{project::project, run::run};
use std::{collections::BTreeSet, fs, path::Path};

pub(crate) fn status(directory: &Path) -> Result<Vec<String>, String> {
    let project = project(directory)?;
    let pathspec = if project.prefix.is_empty() {
        "."
    } else {
        project.prefix.trim_end_matches('/')
    };
    let output = run(
        &project.root,
        &[
            "status",
            "--porcelain=v1",
            "-z",
            "--untracked-files=all",
            "--",
            pathspec,
        ],
    )?;
    let ignored = run(
        &project.root,
        &[
            "ls-files",
            "--cached",
            "--others",
            "--ignored",
            "--exclude-per-directory=.treeignore",
            "-z",
            "--",
            pathspec,
        ],
    )?;
    let excluded: BTreeSet<&[u8]> = ignored.split(|byte| *byte == 0).collect();
    let mut records = output.split(|byte| *byte == 0);
    let mut paths = Vec::new();
    while let Some(record) = records.next() {
        if record.is_empty() {
            continue;
        }
        if record.len() < 4 || record[2] != b' ' {
            return Err("malformed Git status output".to_owned());
        }
        let name = &record[3..];
        if record[..2].iter().any(|code| matches!(code, b'R' | b'C')) {
            records
                .next()
                .ok_or_else(|| "incomplete rename or copy in Git status output".to_owned())?;
        }
        if excluded.contains(name) {
            continue;
        }
        let path =
            std::str::from_utf8(name).map_err(|error| format!("Git path is not UTF-8: {error}"))?;
        let relative = path
            .strip_prefix(&project.prefix)
            .ok_or_else(|| "Git returned a path outside the requested directory".to_owned())?;
        let deleted = record[..2].contains(&b'D');
        let visible = match fs::symlink_metadata(project.root.join(path)) {
            Ok(metadata) => metadata.is_file() || metadata.file_type().is_symlink(),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
            Err(error) => return Err(format!("could not inspect `{path}`: {error}")),
        };
        if deleted || visible {
            paths.push(relative.to_owned());
        }
    }
    paths.sort();
    paths.dedup();
    Ok(paths)
}
