use super::SEPARATOR;

pub(super) fn reserved(line: &str) -> bool {
    matches!(
        line,
        "==> ASK"
            | "==> INPUT"
            | "<== OUTPUT"
            | "==> TREE"
            | "==> READ"
            | "==> EDIT"
            | "==> DELETE"
            | SEPARATOR
    ) || line.starts_with("Step ")
        || line.starts_with('\\')
}
