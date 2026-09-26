pub(super) fn matches_component(pattern: &str, value: &str) -> bool {
    let pattern: Vec<_> = pattern.chars().collect();
    let value: Vec<_> = value.chars().collect();
    super::component_match::matches(&pattern, &value)
}
