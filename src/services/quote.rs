pub(super) fn shell_quote(
    value: &str,
    environment: &std::collections::BTreeMap<String, String>,
) -> String {
    if let Some(variable) = value.strip_prefix('$')
        && environment.contains_key(variable)
        && variable
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_')
    {
        return format!("\"${variable}\"");
    }
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}
