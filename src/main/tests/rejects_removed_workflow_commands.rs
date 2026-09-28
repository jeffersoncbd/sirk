use super::*;

#[test]
fn rejects_removed_workflow_commands() {
    for arguments in [
        vec!["run".to_owned(), "documentation".to_owned()],
        vec!["resume".to_owned(), "history/run-old.log".to_owned()],
        vec!["rpc".to_owned()],
    ] {
        assert!(run(arguments).unwrap_err().contains("invalid command"));
    }
}
