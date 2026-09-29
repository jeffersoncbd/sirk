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
            | "==> WRITE"
            | "==> DELETE"
            | SEPARATOR
    ) || line.starts_with("Step ")
        || line.starts_with('\\')
}
