pub(super) fn matches(pattern: &[char], value: &[char]) -> bool {
    match (pattern, value) {
        ([], []) => true,
        (['*', rest @ ..], _) => {
            matches(rest, value) || (!value.is_empty() && matches(pattern, &value[1..]))
        }
        (['?', rest @ ..], [_, value_rest @ ..]) => matches(rest, value_rest),
        ([expected, rest @ ..], [actual, value_rest @ ..]) if expected == actual => {
            matches(rest, value_rest)
        }
        _ => false,
    }
}
