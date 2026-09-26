pub fn condition(input: &str) -> Result<bool, String> {
    match input.trim() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err("IF input must be true or false".into()),
    }
}
