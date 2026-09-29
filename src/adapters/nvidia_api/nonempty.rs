pub(super) fn nonempty(value: String) -> Option<String> {
    (!value.trim().is_empty()).then_some(value)
}
