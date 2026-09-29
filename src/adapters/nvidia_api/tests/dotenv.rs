use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    adapters::NvidiaApiAdapter,
    harness::{HarnessAdapter, RunRequest},
};

#[test]
fn reads_the_api_key_from_the_execution_directory_dotenv() {
    let directory = std::env::temp_dir().join(format!(
        "nvidia-api-dotenv-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir(&directory).unwrap();
    fs::write(directory.join(".env"), "NVIDIA_API_KEY=dotenv-secret\n").unwrap();
    let request = RunRequest {
        prompt: "Fix the failing test".to_owned(),
        working_directory: PathBuf::from(&directory),
        model: Some("deepseek-ai/deepseek-v4.1-flash".to_owned()),
        event_stream: false,
    };

    let invocation = NvidiaApiAdapter::default().invocation(&request).unwrap();
    assert_eq!(
        invocation.environment.get("NVIDIA_AUTHORIZATION"),
        Some(&"Authorization: Bearer dotenv-secret".to_owned())
    );
    fs::remove_dir_all(directory).unwrap();
}
