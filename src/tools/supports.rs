pub fn supports(name: &str) -> bool {
    matches!(
        name,
        "TREE" | "GIT-STATUS-TREE" | "READ" | "WRITE" | "DELETE" | "EDIT" | "ASK" | "AWAIT"
    )
}
