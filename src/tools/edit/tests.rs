use super::*;

#[path = "tests/append_and_prepend_are_exact_and_versions_are_checked.rs"]
mod append_and_prepend_are_exact_and_versions_are_checked;
#[path = "tests/diff_and_color_rendering_preserve_plain_results.rs"]
mod diff_and_color_rendering_preserve_plain_results;
#[path = "tests/line_edits_preserve_bytes_and_handle_eof.rs"]
mod line_edits_preserve_bytes_and_handle_eof;

fn request(operation: Operation, before: &str, input: &str) -> Request {
    Request {
        path: "file.txt".into(),
        operation,
        line: None,
        start: None,
        end: None,
        version: Some(version(before)),
        old_string: None,
        new_string: None,
        replace_all: false,
        input: input.into(),
    }
}
