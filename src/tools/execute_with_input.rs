use std::path::Path;

use super::{format_paths::format_paths, read, tree::Tree};

pub fn execute_with_input(name: &str, input: &str, directory: &Path) -> Result<String, String> {
    match name {
        "TREE" => {
            let tree = Tree::list(directory)?;
            format_paths("TREE", &tree.files)
        }
        "READ" => read::page(directory, input).map(|page| page.content),
        _ => Err(format!("unknown tool `{name}`")),
    }
}
