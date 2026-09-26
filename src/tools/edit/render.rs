pub(super) fn render(diff: &str, color: bool) -> String {
    let mut rendered = String::new();
    let mut in_hunk = false;
    for line in diff.split_inclusive('\n') {
        if line.starts_with("@@") {
            in_hunk = true;
        }
        let code = if in_hunk && line.starts_with('+') {
            "42"
        } else if in_hunk && line.starts_with('-') {
            "41"
        } else {
            ""
        };
        if color && !code.is_empty() {
            rendered.push_str(&format!("\x1b[{code}m"));
        }
        // Never execute control sequences originating in source files.
        for c in line.trim_end_matches('\n').chars() {
            if c.is_control() && c != '\n' && c != '\t' {
                rendered.extend(c.escape_default());
            } else {
                rendered.push(c);
            }
        }
        if color && !code.is_empty() {
            rendered.push_str("\x1b[0m");
        }
        if line.ends_with('\n') {
            rendered.push('\n');
        }
    }
    rendered
}
