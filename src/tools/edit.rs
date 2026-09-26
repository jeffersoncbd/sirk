//! Workflow-only, version-checked edits with durable preparation and plain diffs.
mod apply_to;
mod commit;
mod diff;
mod display;
mod pending_validate;
mod prepare;
mod read_optional;
mod render;
mod request_validate;
mod sync_parent;
mod target;
mod version;

use serde::{Deserialize, Serialize};

pub use display::display;
pub use version::version;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Operation {
    Insert,
    Delete,
    Replace,
    Prepend,
    Append,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub path: String,
    pub operation: Operation,
    pub line: Option<usize>,
    pub start: Option<usize>,
    pub end: Option<usize>,
    pub version: Option<String>,
    pub input: String,
}

/// Stored in the existing INPUT block. Preparation is committed before mutation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pending {
    pub request: Request,
    pub before: Option<String>,
    #[serde(default)]
    pub was_missing: bool,
}

#[cfg(test)]
mod tests {
    use super::render::render;
    use super::*;

    fn request(operation: Operation, before: &str, input: &str) -> Request {
        Request {
            path: "file.txt".into(),
            operation,
            line: None,
            start: None,
            end: None,
            version: Some(version(before)),
            input: input.into(),
        }
    }

    #[test]
    fn line_edits_preserve_bytes_and_handle_eof() {
        let before = "á\r\n}\r\nlast";
        let mut edit = request(Operation::Insert, before, "new\n");
        edit.line = Some(2);
        assert_eq!(edit.apply_to(before).unwrap(), "á\r\nnew\n}\r\nlast");
        edit.line = Some(4);
        assert_eq!(edit.apply_to(before).unwrap(), "á\r\n}\r\nlastnew\n");
        edit.line = Some(5);
        assert!(edit.apply_to(before).is_err());
        edit = request(Operation::Delete, before, "");
        edit.start = Some(2);
        edit.end = Some(2);
        assert_eq!(edit.apply_to(before).unwrap(), "á\r\nlast");
        edit.operation = Operation::Replace;
        edit.end = Some(3);
        edit.input = "replacement".into();
        assert_eq!(edit.apply_to(before).unwrap(), "á\r\nreplacement");
        edit = request(Operation::Insert, "", "first");
        edit.line = Some(1);
        assert_eq!(edit.apply_to("").unwrap(), "first");
        edit = request(Operation::Insert, "a\n", "b\n");
        edit.line = Some(2);
        assert_eq!(edit.apply_to("a\n").unwrap(), "a\nb\n");
    }

    #[test]
    fn append_and_prepend_are_exact_and_versions_are_checked() {
        for (operation, expected) in [
            (Operation::Append, "oldnew"),
            (Operation::Prepend, "newold"),
        ] {
            let mut edit = request(operation, "old", "new");
            edit.version = None;
            assert_eq!(edit.apply_to("old").unwrap(), expected);
            assert_eq!(edit.apply_to("").unwrap(), "new");
        }
        let mut edit = request(Operation::Delete, "old", "");
        edit.start = Some(1);
        edit.end = Some(1);
        assert!(
            edit.apply_to("changed")
                .unwrap_err()
                .contains("version conflict")
        );
        assert_eq!(edit.apply_to("old").unwrap(), "");
        edit.version = None;
        assert!(edit.apply_to("old").is_err());
    }

    #[test]
    fn diff_and_color_rendering_preserve_plain_results() {
        let mut request = request(Operation::Replace, "a\nold\n", "new\n");
        request.start = Some(2);
        request.end = Some(2);
        let pending = Pending {
            request,
            before: Some("a\nold\n".into()),
            was_missing: false,
        };
        let diff = pending.diff().unwrap();
        assert!(diff.contains("-old\n+new\n"));
        assert_eq!(render(&diff, false), diff);
        assert!(render(&diff, true).contains("\x1b[41m-old\x1b[0m\n\x1b[42m+new\x1b[0m"));
        assert!(!render("\x1b[2J", false).contains('\x1b'));
        let mut pending = pending;
        pending.request.input = "new".into();
        assert!(
            pending
                .diff()
                .unwrap()
                .contains("No newline at end of file")
        );
    }
}
