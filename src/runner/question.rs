pub(super) fn question(text: &str) -> Option<&str> {
    text.trim_start().strip_prefix("ASK:").map(str::trim)
}
