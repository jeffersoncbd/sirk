use std::path::Path;

use super::{format_paths::format_paths, git_status_tree::GitStatusTree, read, tree::Tree};

pub fn execute_with_input(name: &str, input: &str, directory: &Path) -> Result<String, String> {
    match name {
        "TREE" => {
            let tree = Tree::list(directory)?;
            format_paths("TREE", &tree.files)
        }
        "GIT-STATUS-TREE" => {
            let tree = GitStatusTree::list(directory)?;
            format_paths("GIT-STATUS-TREE", &tree.files)
        }
        "READ" => read::read(directory, input),
        _ => Err(format!("unknown tool `{name}`")),
    }
}
