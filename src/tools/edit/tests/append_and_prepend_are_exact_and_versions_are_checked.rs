use super::*;

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
