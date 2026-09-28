use super::*;

#[test]
fn line_edits_preserve_bytes_and_handle_eof() {
    let before = "café\r\n}\r\nlast";
    let mut edit = request(Operation::Insert, before, "new\n");
    edit.line = Some(2);
    assert_eq!(edit.apply_to(before).unwrap(), "café\r\nnew\n}\r\nlast");
    edit.line = Some(4);
    assert_eq!(edit.apply_to(before).unwrap(), "café\r\n}\r\nlastnew\n");
    edit.line = Some(5);
    assert!(edit.apply_to(before).is_err());
    edit = request(Operation::Delete, before, "");
    edit.start = Some(2);
    edit.end = Some(2);
    assert_eq!(edit.apply_to(before).unwrap(), "café\r\nlast");
    edit.operation = Operation::Replace;
    edit.end = Some(3);
    edit.input = "replacement".into();
    assert_eq!(edit.apply_to(before).unwrap(), "café\r\nreplacement");
    edit = request(Operation::Insert, "", "first");
    edit.line = Some(1);
    assert_eq!(edit.apply_to("").unwrap(), "first");
    edit = request(Operation::Insert, "a\n", "b\n");
    edit.line = Some(2);
    assert_eq!(edit.apply_to("a\n").unwrap(), "a\nb\n");
}
