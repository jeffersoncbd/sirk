pub(super) fn matches_components(pattern: &[&str], path: &[&str], prefix: bool) -> bool {
    match (pattern, path) {
        ([], []) => true,
        ([], _) => prefix,
        (["**", rest @ ..], _) => {
            matches_components(rest, path, prefix)
                || (!path.is_empty() && matches_components(pattern, &path[1..], prefix))
        }
        ([part, rest @ ..], [path_part, path_rest @ ..]) => {
            super::component::matches_component(part, path_part)
                && matches_components(rest, path_rest, prefix)
        }
        _ => false,
    }
}
