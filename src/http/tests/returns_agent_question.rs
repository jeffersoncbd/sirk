use super::{flow::flow, request::request};
use axum::body::to_bytes;
use serde_json::Value;
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    time::{SystemTime, UNIX_EPOCH},
};

#[tokio::test]
async fn returns_an_authorized_agent_question() {
    let directory = std::env::temp_dir().join(format!(
        "sirk-http-ask-test-{}-{}",
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
            "---\nadapter: codex\ncall_prefix: '{}'\nASK_TOOL: allow\n---\nAsk one question.",
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
    assert_eq!(response.status(), 200);
    let response = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let response: Value = serde_json::from_slice(&response).unwrap();
    assert_eq!(response["ask"], "What is your name?");
    let conversation_id = response["conversationId"].as_str().unwrap();
    assert_eq!(response.as_object().unwrap().len(), 2);
    let transcript =
        fs::read_to_string(directory.join("history").join(&flow_id).join("flow.log")).unwrap();
    assert!(transcript.contains("<== OUTPUT\nASK: What is your name?"));
    let conversation = fs::read_to_string(
        directory
            .join("history")
            .join(&flow_id)
            .join("conversations")
            .join(format!("{conversation_id}.json")),
    )
    .unwrap();
    let conversation: Value = serde_json::from_str(&conversation).unwrap();
    assert_eq!(conversation["status"], "awaiting_user");
    assert_eq!(conversation["messages"].as_array().unwrap().len(), 2);
    fs::remove_dir_all(directory).unwrap();
}
