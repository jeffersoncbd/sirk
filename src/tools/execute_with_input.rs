use std::path::Path;

use super::{format_paths::format_paths, read, tree::Tree};

pub fn execute_with_input(name: &str, input: &str, directory: &Path) -> Result<String, String> {
    match name {
        "TREE" => {
            let tree = Tree::list(directory)?;
            format_paths("TREE", &tree.files)
        }
        "READ" => read::read(directory, input),
        _ => Err(format!("unknown tool `{name}`")),
    }
}
