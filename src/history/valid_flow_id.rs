pub fn valid_flow_id(value: &str) -> bool {
    value.strip_prefix("flow-").is_some_and(|value| {
        !value.is_empty()
            && value
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() || byte == b'-')
    })
}
