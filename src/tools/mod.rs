//! Shared tool dispatch for workflow steps and agent requests.
pub mod new_agent;
pub mod read;
pub mod tree;

pub fn supports(name: &str) -> bool {
    matches!(name, "TREE" | "READ")
}

/// Only standalone tool requests are interpreted as control messages.
pub fn request(text: &str) -> Option<(&str, &str)> {
    let text = text.trim();
    if text == "TREE" {
        return Some(("TREE", ""));
    }
    let path = text
        .strip_prefix("READ: ")
        .or_else(|| (text == "READ:").then_some(""))?;
    if path.contains(['\n', '\r']) {
        return None;
    }
    Some(("READ", path))
}

pub fn execute(name: &str, directory: &std::path::Path) -> Result<String, String> {
    execute_with_input(name, "", directory)
}

pub fn execute_with_input(
    name: &str,
    input: &str,
    directory: &std::path::Path,
) -> Result<String, String> {
    match name {
        "TREE" => {
            let tree = tree::Tree::list(directory)?;
            format_tree(&tree.files)
        }
        "READ" => read::read(directory, input),
        _ => Err(format!("unknown tool `{name}`")),
    }
}

fn format_tree(files: &[std::path::PathBuf]) -> Result<String, String> {
    let paths: Vec<&str> = files
        .iter()
        .map(|path| {
            path.to_str()
                .ok_or("TREE cannot represent a non-UTF-8 path in a JSON array")
        })
        .collect::<Result<_, _>>()?;
    serde_json::to_string_pretty(&paths)
        .map(|json| format!("{json}\n"))
        .map_err(|error| format!("TREE could not serialize paths: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tree_is_a_json_array_with_one_path_per_line() {
        let paths = [
            "src/main.rs",
            "quotes\"and\\slashes",
            "line\nbreak",
            "tab\tand\u{1}",
            "ação.rs",
        ];
        let files: Vec<_> = paths.iter().map(std::path::PathBuf::from).collect();
        let output = format_tree(&files).unwrap();
        assert_eq!(serde_json::from_str::<Vec<String>>(&output).unwrap(), paths);
        assert_eq!(output.lines().count(), paths.len() + 2);
        assert_eq!(format_tree(&[]).unwrap(), "[]\n");
    }
    #[test]
    fn parses_only_standalone_requests() {
        assert_eq!(
            request("READ: src/my file.rs\n"),
            Some(("READ", "src/my file.rs"))
        );
        assert_eq!(request("TREE\n"), Some(("TREE", "")));
        for text in [
            "Please READ: file",
            "READ: file\nMore text",
            "```\nREAD: file\n```",
            "read: file",
        ] {
            assert_eq!(request(text), None);
        }
    }
}
