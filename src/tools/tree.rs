//! TREE: recursive file inventory using Git's own ignore and index semantics.
use crate::services::{BashService, Invocation};
use std::{
    collections::BTreeSet,
    fs, io,
    path::{Path, PathBuf},
};

/// Paths are sorted, unique, and relative to `root`.
#[derive(Debug, PartialEq, Eq)]
pub struct Tree {
    pub root: PathBuf,
    pub files: Vec<PathBuf>,
}

impl Tree {
    /// List existing tracked and non-ignored untracked files below a directory
    /// in a Git working tree. Git is required. This does not modify the index.
    ///
    /// Git metadata, empty directories, and submodule contents are not listed.
    /// Symbolic links are listed as entries, never traversed. Like Git, ignore
    /// .gitignore rules do not remove files that are already tracked. Additional
    /// .treeignore rules apply to both tracked and untracked files.
    pub fn list(directory: &Path) -> Result<Self, String> {
        let root = directory
            .canonicalize()
            .map_err(|e| format!("TREE cannot resolve `{}`: {e}", directory.display()))?;
        if !root.is_dir() {
            return Err(format!("TREE root is not a directory: {}", root.display()));
        }
        let invocation = Invocation {
            program: "git".into(),
            arguments: [
                "ls-files",
                "--cached",
                "--others",
                "--exclude-standard",
                "-z",
                "--",
                ".",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            working_directory: root.clone(),
        };
        let result = BashService::default()
            .execute_bytes_to(&invocation, &mut io::sink())
            .map_err(|e| format!("TREE could not execute Git: {e}"))?;
        if !result.status.success() {
            return Err(format!(
                "TREE requires an accessible Git working tree; git ls-files exited with {}",
                result.status
            ));
        }
        // Ask Git to evaluate only TREE-specific excludes, then subtract them.
        // Keeping the filters separate prevents a .treeignore negation from
        // reintroducing an untracked file excluded by .gitignore.
        let tree_ignored = BashService::default()
            .execute_bytes_to(
                &Invocation {
                    program: "git".into(),
                    arguments: [
                        "ls-files",
                        "--cached",
                        "--others",
                        "--ignored",
                        "--exclude-per-directory=.treeignore",
                        "-z",
                        "--",
                        ".",
                    ]
                    .into_iter()
                    .map(str::to_owned)
                    .collect(),
                    working_directory: root.clone(),
                },
                &mut io::sink(),
            )
            .map_err(|e| format!("TREE could not evaluate .treeignore: {e}"))?;
        if !tree_ignored.status.success() {
            return Err(format!(
                "TREE .treeignore evaluation failed: {}",
                tree_ignored.status
            ));
        }
        let excluded: BTreeSet<&[u8]> = tree_ignored.stdout.split(|byte| *byte == 0).collect();
        let mut files = Vec::new();
        for bytes in result
            .stdout
            .split(|byte| *byte == 0)
            .filter(|bytes| !bytes.is_empty())
        {
            if excluded.contains(bytes) {
                continue;
            }
            let path = path_from_bytes(bytes)?;
            match fs::symlink_metadata(root.join(&path)) {
                Ok(metadata) if metadata.is_file() || metadata.file_type().is_symlink() => {
                    files.push(path)
                }
                // Tracked deletions and directory entries (gitlinks) are not files.
                Ok(_) => (),
                Err(error) if error.kind() == io::ErrorKind::NotFound => (),
                Err(error) => {
                    return Err(format!(
                        "TREE cannot inspect `{}`: {error}",
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

#[cfg(unix)]
fn path_from_bytes(bytes: &[u8]) -> Result<PathBuf, String> {
    use std::os::unix::ffi::OsStrExt;
    Ok(PathBuf::from(std::ffi::OsStr::from_bytes(bytes)))
}

#[cfg(not(unix))]
fn path_from_bytes(bytes: &[u8]) -> Result<PathBuf, String> {
    std::str::from_utf8(bytes)
        .map(PathBuf::from)
        .map_err(|e| format!("TREE received a path unsupported on this platform: {e}"))
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
                "tree-{}-{stamp}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            )));
            fs::create_dir(&project.0).unwrap();
            project.git(&["init", "--quiet"]);
            // Isolate the fixture from the developer's global excludes.
            project.write(".git/test-excludes", "");
            project.git(&[
                "config",
                "core.excludesFile",
                project.0.join(".git/test-excludes").to_str().unwrap(),
            ]);
            project
        }
        fn git(&self, args: &[&str]) {
            let result = BashService::default()
                .execute_to(
                    &Invocation {
                        program: "git".into(),
                        arguments: args.iter().map(|s| s.to_string()).collect(),
                        working_directory: self.0.clone(),
                    },
                    &mut io::sink(),
                )
                .unwrap();
            assert!(result.status.success());
        }
        fn write(&self, name: &str, content: &str) {
            let path = self.0.join(name);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, content).unwrap();
        }
        fn paths(&self) -> Vec<PathBuf> {
            Tree::list(&self.0).unwrap().files
        }
    }
    impl Drop for Project {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn obeys_nested_patterns_negation_anchoring_and_standard_excludes() {
        let project = Project::new();
        project.write(".gitignore", "build/\n*.log\n!keep.log\n/root.txt\n**/cache/**\nblocked/\n!blocked/keep.txt\n\\#literal\n\\!literal\n");
        project.write("src/.gitignore", "*.tmp\n!keep.tmp\n");
        project.write(".git/info/exclude", "local.txt\n");
        project.write(".git/test-excludes", "global.txt\n");
        let visible = [
            ".gitignore",
            "src/.gitignore",
            "keep.log",
            "src/root.txt",
            "src/keep.tmp",
            "src/main.rs",
            ".hidden",
            "space name.txt",
            "line\nbreak.txt",
        ];
        for file in &visible[2..] {
            project.write(file, "");
        }
        for file in [
            "build/deep/file",
            "debug.log",
            "root.txt",
            "src/delete.tmp",
            "src/cache/file",
            "blocked/keep.txt",
            "local.txt",
            "global.txt",
            "#literal",
            "!literal",
        ] {
            project.write(file, "");
        }
        let mut expected: Vec<_> = visible.iter().map(PathBuf::from).collect();
        expected.sort();
        assert_eq!(project.paths(), expected);
        let subtree = Tree::list(&project.0.join("src")).unwrap();
        assert_eq!(
            subtree.files,
            [".gitignore", "keep.tmp", "main.rs", "root.txt"].map(PathBuf::from)
        );
    }

    #[test]
    fn treeignore_filters_tracked_files_without_changing_the_index() {
        let project = Project::new();
        for path in [".agents/planner.md", "Cargo.lock", "keep.rs"] {
            project.write(path, "");
        }
        project.git(&["add", "."]);
        let before = fs::read(project.0.join(".git/index")).unwrap();
        project.write(
            ".treeignore",
            "/.agents\nCargo.lock\n*.tmp\n!keep.tmp\n!hidden.log\n",
        );
        project.write(".gitignore", "*.log\n");
        for path in ["skip.tmp", "keep.tmp", "hidden.log"] {
            project.write(path, "");
        }
        assert_eq!(
            project.paths(),
            [".gitignore", ".treeignore", "keep.rs", "keep.tmp"].map(PathBuf::from)
        );
        assert_eq!(before, fs::read(project.0.join(".git/index")).unwrap());
    }

    #[test]
    fn treeignore_uses_nested_git_patterns_and_applies_to_subtrees() {
        let project = Project::new();
        project.write(
            ".treeignore",
            "/root.txt\n*.tmp\nblocked/\n!blocked/keep.txt\n**/cache/**\n",
        );
        project.write("src/.treeignore", "!keep.tmp\nlocal.txt\n");
        for path in [
            "root.txt",
            "src/root.txt",
            "src/keep.tmp",
            "src/drop.tmp",
            "src/local.txt",
            "blocked/keep.txt",
            "src/cache/file",
        ] {
            project.write(path, "");
        }
        project.git(&["add", "."]);
        assert_eq!(
            project.paths(),
            [
                ".treeignore",
                "src/.treeignore",
                "src/keep.tmp",
                "src/root.txt"
            ]
            .map(PathBuf::from)
        );
        assert_eq!(
            Tree::list(&project.0.join("src")).unwrap().files,
            [".treeignore", "keep.tmp", "root.txt"].map(PathBuf::from)
        );
    }

    #[test]
    fn tracked_ignored_files_remain_but_deleted_files_do_not() {
        let project = Project::new();
        project.write("tracked.log", "tracked");
        project.write("deleted.txt", "deleted");
        project.git(&["add", "--", "tracked.log", "deleted.txt"]);
        project.write(".gitignore", "*.log\n");
        project.write("untracked.log", "ignored");
        fs::remove_file(project.0.join("deleted.txt")).unwrap();
        assert_eq!(
            project.paths(),
            [".gitignore", "tracked.log"].map(PathBuf::from)
        );
    }

    #[cfg(unix)]
    #[test]
    fn preserves_non_utf8_paths_and_does_not_follow_links() {
        use std::os::unix::{ffi::OsStrExt, fs::symlink};
        let project = Project::new();
        let name = PathBuf::from(std::ffi::OsStr::from_bytes(b"non-utf8-\xff"));
        fs::write(project.0.join(&name), "").unwrap();
        symlink(".", project.0.join("cycle")).unwrap();
        symlink("missing", project.0.join("broken")).unwrap();
        assert_eq!(
            project.paths(),
            vec![PathBuf::from("broken"), PathBuf::from("cycle"), name]
        );
    }

    #[test]
    fn rejects_missing_root_and_non_repository() {
        let project = Project::new();
        assert!(Tree::list(&project.0.join("missing")).is_err());
        project.write("file", "");
        assert!(Tree::list(&project.0.join("file")).is_err());
        fs::remove_dir_all(project.0.join(".git")).unwrap();
        assert!(Tree::list(&project.0).is_err());
    }
}
