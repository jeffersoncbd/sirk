//! GIT-STATUS-TREE: list changed paths reported by Git status.
mod git;
mod list;
mod path;
mod tree_ignored;

use std::path::PathBuf;

/// Paths are sorted, unique, and relative to `root`.
#[derive(Debug, PartialEq, Eq)]
pub struct GitStatusTree {
    pub root: PathBuf,
    pub files: Vec<PathBuf>,
}
