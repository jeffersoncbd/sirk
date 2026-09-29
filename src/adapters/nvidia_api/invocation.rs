use std::collections::BTreeMap;

use serde_json::json;

use super::{ENDPOINT, NvidiaApiAdapter};
use crate::harness::{HarnessAdapter, HarnessError, Invocation, RunRequest};

pub(super) fn build(
    adapter: &NvidiaApiAdapter,
    request: &RunRequest,
) -> Result<Invocation, HarnessError> {
    let model = request.model.as_ref().ok_or(HarnessError::MissingModel {
        adapter: adapter.id(),
    })?;
    if request.event_stream {
        return Err(HarnessError::UnsupportedOption {
            adapter: adapter.id(),
            option: "event streams",
        });
    }
    let mut environment = BTreeMap::new();
    environment.insert(
        "NVIDIA_AUTHORIZATION".to_owned(),
        format!(
            "Authorization: Bearer {}",
            adapter.api_key(&request.working_directory)?
        ),
    );
    Ok(Invocation {
        program: adapter.executable.clone(),
        arguments: vec![
            "--fail".to_owned(),
            "--show-error".to_owned(),
            "--no-buffer".to_owned(),
            "--request".to_owned(),
            "POST".to_owned(),
            "--header".to_owned(),
            "Content-Type: application/json".to_owned(),
            "--header".to_owned(),
            "$NVIDIA_AUTHORIZATION".to_owned(),
            "--data".to_owned(),
            json!({
                "model": model,
                "messages": [{ "role": "user", "content": request.prompt }],
                "stream": false,
            })
            .to_string(),
            ENDPOINT.to_owned(),
        ],
        working_directory: request.working_directory.clone(),
        environment,
    })
}
