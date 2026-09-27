use super::*;

#[test]
fn translates_generic_options_to_opencode_run() {
    let request = RunRequest {
        prompt: "Fix the failing test".to_owned(),
        working_directory: PathBuf::from("/workspace"),
        model: Some("openai/gpt-5.3-codex".to_owned()),
        event_stream: true,
    };

    assert_eq!(
        OpenCodeAdapter::default().invocation(&request).unwrap(),
        Invocation {
            program: "opencode".to_owned(),
            arguments: vec![
                "run",
                "--title",
                "S.I.R.K.",
                "--format",
                "json",
                "--model",
                "openai/gpt-5.3-codex",
                "--",
                "Fix the failing test",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            working_directory: PathBuf::from("/workspace"),
            environment: Default::default(),
        }
    );
}
