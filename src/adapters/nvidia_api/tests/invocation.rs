use std::path::PathBuf;

use crate::{
    adapters::NvidiaApiAdapter,
    harness::{HarnessAdapter, RunRequest},
};

#[test]
fn posts_a_non_streaming_chat_request_without_exposing_the_api_key() {
    let request = RunRequest {
        prompt: "Fix the failing test".to_owned(),
        working_directory: PathBuf::from("/workspace"),
        model: Some("deepseek-ai/deepseek-v4.1-flash".to_owned()),
        event_stream: false,
    };
    let invocation = NvidiaApiAdapter::new("curl", Some("secret".to_owned()))
        .invocation(&request)
        .unwrap();

    assert_eq!(
        invocation.arguments.last(),
        Some(&"https://integrate.api.nvidia.com/v1/chat/completions".to_owned())
    );
    assert!(invocation.arguments.contains(&"--no-buffer".to_owned()));
    assert!(invocation.arguments.iter().any(|argument| {
        argument.contains("\"messages\"") && argument.contains("\"stream\":false")
    }));
    assert!(
        !invocation
            .arguments
            .iter()
            .any(|argument| argument.contains("secret"))
    );
    assert_eq!(
        invocation.environment.get("NVIDIA_AUTHORIZATION"),
        Some(&"Authorization: Bearer secret".to_owned())
    );
}
