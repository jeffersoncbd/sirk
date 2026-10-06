use super::{flow::flow, request::request};
use axum::body::to_bytes;
use serde_json::Value;
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    time::{SystemTime, UNIX_EPOCH},
};

#[tokio::test]
async fn rejects_an_agent_question_without_permission() {
    let directory = std::env::temp_dir().join(format!(
        "sirk-http-unauthorized-ask-test-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(directory.join(".agents")).unwrap();
    let adapter = directory.join("adapter");
    fs::write(
        &adapter,
        "#!/bin/sh\nprintf '%s' 'ASK: What is your name?'\n",
    )
    .unwrap();
    let mut permissions = fs::metadata(&adapter).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&adapter, permissions).unwrap();
    fs::write(
        directory.join(".agents/interviewer.md"),
        format!(
            "---\nadapter: codex\ncall_prefix: '{}'\n---\nAsk one question.",
            adapter.display()
        ),
    )
    .unwrap();
    let body = serde_json::json!({
        "directory": directory,
        "agent": "interviewer",
        "input": "Collect the profile."
    })
    .to_string();
    let flow_id = flow(&directory).await;
    let response = request("POST", "/v1/agent/run", body, Some(&flow_id)).await;
    assert_eq!(response.status(), 500);
    let response = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let response: Value = serde_json::from_slice(&response).unwrap();
    assert_eq!(
        response["error"],
        "agent requested ASK_TOOL without permission"
    );
    fs::remove_dir_all(directory).unwrap();
}
