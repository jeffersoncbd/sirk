pub(crate) fn valid_conversation_id(value: &str) -> bool {
    value.strip_prefix("conversation-").is_some_and(|value| {
        !value.is_empty()
            && value
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() || byte == b'-')
    })
}
