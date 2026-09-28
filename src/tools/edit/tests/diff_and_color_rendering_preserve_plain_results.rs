use super::*;
use crate::tools::edit::render::render;

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
