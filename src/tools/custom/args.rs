pub fn arguments(input: &str) -> Result<Vec<String>, String> {
    serde_json::from_str(input)
        .map_err(|error| format!("CUSTOM-TOOL requires a list of string arguments: {error}"))
}
