pub(super) fn format_paths(tool: &str, files: &[std::path::PathBuf]) -> Result<String, String> {
    let paths: Vec<&str> = files
        .iter()
        .map(|path| {
            path.to_str()
                .ok_or_else(|| format!("{tool} cannot represent a non-UTF-8 path in a JSON array"))
        })
        .collect::<Result<_, _>>()?;
    serde_json::to_string_pretty(&paths)
        .map(|json| format!("{json}\n"))
        .map_err(|error| format!("{tool} could not serialize paths: {error}"))
}

#[cfg(test)]
mod tests {
    use super::format_paths;

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
        let output = format_paths("TREE", &files).unwrap();
        assert_eq!(serde_json::from_str::<Vec<String>>(&output).unwrap(), paths);
        assert_eq!(output.lines().count(), paths.len() + 2);
        assert_eq!(format_paths("TREE", &[]).unwrap(), "[]\n");
    }
}
