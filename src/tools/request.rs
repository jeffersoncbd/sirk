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

#[cfg(test)]
mod tests {
    use super::request;

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
