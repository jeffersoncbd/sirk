use std::path::PathBuf;

use crate::{
    adapters::NvidiaApiAdapter,
    harness::{HarnessAdapter, HarnessError, RunRequest},
};

#[test]
fn requires_a_model_and_rejects_event_streams() {
    let adapter = NvidiaApiAdapter::new("curl", Some("key".into()));
    let request = RunRequest {
        prompt: "Fix the failing test".to_owned(),
        working_directory: PathBuf::from("/workspace"),
        model: None,
        event_stream: false,
    };
    assert_eq!(
        adapter.invocation(&request),
        Err(HarnessError::MissingModel {
            adapter: "nvidia-api"
        })
    );
    let event_stream = RunRequest {
        model: Some("deepseek-ai/deepseek-v4.1-flash".to_owned()),
        event_stream: true,
        ..request
    };
    assert_eq!(
        adapter.invocation(&event_stream),
        Err(HarnessError::UnsupportedOption {
            adapter: "nvidia-api",
            option: "event streams",
        })
    );
}
