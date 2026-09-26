pub fn loop_items(input: &str) -> Result<Vec<String>, String> {
    serde_json::from_str(input)
        .map_err(|error| format!("LOOP requires a JSON array of strings: {error}"))
}
