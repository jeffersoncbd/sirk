//! TREE: recursive file inventory using Git's own ignore and index semantics.
mod list;
mod path;

use std::path::PathBuf;

/// Paths are sorted, unique, and relative to `root`.
#[derive(Debug, PartialEq, Eq)]
pub struct Tree {
    pub root: PathBuf,
    pub files: Vec<PathBuf>,
}
