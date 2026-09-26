pub(super) fn matches_ignore(pattern: &str, path: &str) -> bool {
    let pattern = pattern.trim_start_matches('/');
    let directory = pattern.strip_suffix('/').unwrap_or(pattern);
    let path_parts: Vec<_> = path.split('/').collect();
    let pattern_parts: Vec<_> = directory.split('/').collect();
    if pattern_parts.len() == 1 {
        return path_parts
            .iter()
            .any(|part| super::component::matches_component(pattern_parts[0], part));
    }
    if pattern.ends_with('/') {
        return super::components::matches_components(&pattern_parts, &path_parts, true);
    }
    super::components::matches_components(&pattern_parts, &path_parts, false)
}
