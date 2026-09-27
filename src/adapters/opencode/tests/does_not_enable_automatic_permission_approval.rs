use super::*;

#[test]
fn does_not_enable_automatic_permission_approval() {
    let request = RunRequest {
        prompt: "Inspect the project".to_owned(),
        working_directory: PathBuf::from("/workspace"),
        model: None,
        event_stream: false,
    };

    let invocation = OpenCodeAdapter::default().invocation(&request).unwrap();

    assert!(
        !invocation
            .arguments
            .iter()
            .any(|argument| argument == "--auto")
    );
}
