//! GIT-STATUS-TREE: list existing paths reported by Git status.
use crate::services::{BashService, Invocation};
use std::{
    collections::BTreeSet,
    fs, io,
    path::{Path, PathBuf},
    process::ExitStatus,
};

struct GitOutput {
    status: ExitStatus,
    stdout: Vec<u8>,
}

/// Paths are sorted, unique, and relative to `root`.
#[derive(Debug, PartialEq, Eq)]
pub struct GitStatusTree {
    pub root: PathBuf,
    pub files: Vec<PathBuf>,
}

impl GitStatusTree {
    /// List existing modified, added, renamed, copied, unmerged, and untracked
    /// files below a directory in a Git working tree. Deleted files, ignored
    /// files, directories, and submodule contents are omitted. Additional
    /// `.treeignore` rules apply to every status entry.
    pub fn list(directory: &Path) -> Result<Self, String> {
        let root = directory.canonicalize().map_err(|error| {
            format!(
                "GIT-STATUS-TREE cannot resolve `{}`: {error}",
                directory.display()
            )
        })?;
        if !root.is_dir() {
            return Err(format!(
                "GIT-STATUS-TREE root is not a directory: {}",
                root.display()
            ));
        }

        let prefix_result = git(&root, &["rev-parse", "--show-prefix"])?;
        if !prefix_result.status.success() {
            return Err(format!(
                "GIT-STATUS-TREE requires an accessible Git working tree; git rev-parse exited with {}",
                prefix_result.status
            ));
        }
        let prefix = prefix_result
            .stdout
            .strip_suffix(b"\n")
            .unwrap_or(&prefix_result.stdout);

        let status = git(
            &root,
            &[
                "status",
                "--porcelain=v1",
                "-z",
                "--untracked-files=all",
                "--",
                ".",
            ],
        )?;
        if !status.status.success() {
            return Err(format!(
                "GIT-STATUS-TREE could not read Git status; git status exited with {}",
                status.status
            ));
        }

        let excluded = tree_ignored(&root)?;
        let mut files = Vec::new();
        let mut records = status.stdout.split(|byte| *byte == 0);
        while let Some(record) = records.next() {
            if record.is_empty() {
                continue;
            }
            if record.len() < 4 || record[2] != b' ' {
                return Err("GIT-STATUS-TREE received malformed Git status output".into());
            }
            let status_code = &record[..2];
            let repository_path = &record[3..];
            if status_code.iter().any(|code| matches!(code, b'R' | b'C')) {
                records.next().ok_or(
                    "GIT-STATUS-TREE received an incomplete rename or copy from Git status",
                )?;
            }
            let relative = repository_path
                .strip_prefix(prefix)
                .ok_or("GIT-STATUS-TREE received a path outside the execution directory")?;
            if relative.is_empty() || excluded.contains(relative) {
                continue;
            }
            let path = path_from_bytes(relative)?;
            match fs::symlink_metadata(root.join(&path)) {
                Ok(metadata) if metadata.is_file() || metadata.file_type().is_symlink() => {
                    files.push(path)
                }
                Ok(_) => (),
                Err(error) if error.kind() == io::ErrorKind::NotFound => (),
                Err(error) => {
                    return Err(format!(
                        "GIT-STATUS-TREE cannot inspect `{}`: {error}",
                        root.join(&path).display()
                    ));
                }
            }
        }
        files.sort();
        files.dedup();
        Ok(Self { root, files })
    }
}

fn tree_ignored(root: &Path) -> Result<BTreeSet<Vec<u8>>, String> {
    let result = git(
        root,
        &[
            "ls-files",
            "--cached",
            "--others",
            "--ignored",
            "--exclude-per-directory=.treeignore",
            "-z",
            "--",
            ".",
        ],
    )?;
    if !result.status.success() {
        return Err(format!(
            "GIT-STATUS-TREE .treeignore evaluation failed: {}",
            result.status
        ));
    }
    Ok(result
        .stdout
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(<[u8]>::to_vec)
        .collect())
}

fn git(root: &Path, arguments: &[&str]) -> Result<GitOutput, String> {
    let result = BashService::default()
        .execute_bytes_to(
            &Invocation {
                program: "git".into(),
                arguments: arguments.iter().map(|value| (*value).into()).collect(),
                working_directory: root.to_owned(),
            },
            &mut io::sink(),
        )
        .map_err(|error| format!("GIT-STATUS-TREE could not execute Git: {error}"))?;
    Ok(GitOutput {
        status: result.status,
        stdout: result.stdout,
    })
}

#[cfg(unix)]
fn path_from_bytes(bytes: &[u8]) -> Result<PathBuf, String> {
    use std::os::unix::ffi::OsStrExt;
    Ok(PathBuf::from(std::ffi::OsStr::from_bytes(bytes)))
}

#[cfg(not(unix))]
fn path_from_bytes(bytes: &[u8]) -> Result<PathBuf, String> {
    std::str::from_utf8(bytes)
        .map(PathBuf::from)
        .map_err(|error| {
            format!("GIT-STATUS-TREE received a path unsupported on this platform: {error}")
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        sync::atomic::{AtomicUsize, Ordering},
        time::{SystemTime, UNIX_EPOCH},
    };

    struct Project(PathBuf);

    impl Project {
        fn new() -> Self {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let stamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let project = Self(std::env::temp_dir().join(format!(
                "git-status-tree-{}-{stamp}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            )));
            fs::create_dir(&project.0).unwrap();
            project.git(&["init", "--quiet"]);
            project.git(&["config", "user.email", "test@example.com"]);
            project.git(&["config", "user.name", "Test"]);
            project.write(".git/test-excludes", "");
            project.git(&[
                "config",
                "core.excludesFile",
                project.0.join(".git/test-excludes").to_str().unwrap(),
            ]);
            project
        }

        fn git(&self, arguments: &[&str]) {
            let result = git(&self.0, arguments).unwrap();
            assert!(result.status.success(), "git {arguments:?} failed");
        }

        fn write(&self, name: &str, content: &str) {
            let path = self.0.join(name);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, content).unwrap();
        }

        fn paths(&self) -> Vec<PathBuf> {
            GitStatusTree::list(&self.0).unwrap().files
        }
    }

    impl Drop for Project {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn lists_existing_status_files_and_applies_treeignore() {
        let project = Project::new();
        for path in [
            "clean.txt",
            "modified.txt",
            "staged.txt",
            "deleted.txt",
            "renamed-old.txt",
            "hidden-tracked.txt",
        ] {
            project.write(path, "old");
        }
        project.git(&["add", "."]);
        project.git(&["commit", "--quiet", "-m", "initial"]);

        project.write("modified.txt", "new");
        project.write("staged.txt", "new");
        project.git(&["add", "staged.txt"]);
        fs::remove_file(project.0.join("deleted.txt")).unwrap();
        project.git(&["mv", "renamed-old.txt", "renamed.txt"]);
        project.write("added.txt", "new");
        project.git(&["add", "added.txt"]);
        project.write("untracked.txt", "new");
        project.write("ignored.log", "ignored");
        project.write("hidden-untracked.txt", "hidden");
        project.write(".gitignore", "*.log\n");
        project.write(".treeignore", "hidden-tracked.txt\nhidden-untracked.txt\n");

        assert_eq!(
            project.paths(),
            [
                ".gitignore",
                ".treeignore",
                "added.txt",
                "modified.txt",
                "renamed.txt",
                "staged.txt",
                "untracked.txt",
            ]
            .map(PathBuf::from)
        );
    }

    #[test]
    fn returns_paths_relative_to_a_subdirectory() {
        let project = Project::new();
        project.write("root.txt", "old");
        project.write("src/tracked.txt", "old");
        project.git(&["add", "."]);
        project.git(&["commit", "--quiet", "-m", "initial"]);
        project.write("root.txt", "new");
        project.write("src/tracked.txt", "new");
        project.write("src/new.txt", "new");

        assert_eq!(
            GitStatusTree::list(&project.0.join("src")).unwrap().files,
            ["new.txt", "tracked.txt"].map(PathBuf::from)
        );
    }

    #[test]
    fn rejects_missing_root_and_non_repository() {
        let project = Project::new();
        assert!(GitStatusTree::list(&project.0.join("missing")).is_err());
        fs::remove_dir_all(project.0.join(".git")).unwrap();
        assert!(GitStatusTree::list(&project.0).is_err());
    }
}
