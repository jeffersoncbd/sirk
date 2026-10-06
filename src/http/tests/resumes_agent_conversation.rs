use super::{flow::flow, request::request};
use axum::body::to_bytes;
use serde_json::Value;
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    time::{SystemTime, UNIX_EPOCH},
};

#[tokio::test]
async fn resumes_an_agent_conversation_from_its_persisted_messages() {
    let directory = std::env::temp_dir().join(format!(
        "sirk-http-conversation-test-{}-{}",
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
        "#!/bin/sh\ncase \"$*\" in\n  *'ASK: What is your name?'*'User:'*'Ana'*) printf '%s' 'Profile complete.' ;;\n  *) printf '%s' 'ASK: What is your name?' ;;\nesac\n",
    )
    .unwrap();
    let mut permissions = fs::metadata(&adapter).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&adapter, permissions).unwrap();
    fs::write(
        directory.join(".agents/interviewer.md"),
        format!(
            "---\nadapter: codex\ncall_prefix: '{}'\nASK_TOOL: allow\n---\nCollect a profile one question at a time.",
            adapter.display()
        ),
    )
    .unwrap();
    let flow_id = flow(&directory).await;
    let first = request(
        "POST",
        "/v1/agent/run",
        serde_json::json!({
            "directory": directory,
            "agent": "interviewer",
            "input": "Start the interview."
        })
        .to_string(),
        Some(&flow_id),
    )
    .await;
    assert_eq!(first.status(), 200);
    let first = to_bytes(first.into_body(), usize::MAX).await.unwrap();
    let first: Value = serde_json::from_slice(&first).unwrap();
    assert_eq!(first["ask"], "What is your name?");
    let conversation_id = first["conversationId"].as_str().unwrap();
    let second = request(
        "POST",
        "/v1/agent/run",
        serde_json::json!({
            "directory": directory,
            "agent": "interviewer",
            "input": "Ana",
            "conversationId": conversation_id
        })
        .to_string(),
        Some(&flow_id),
    )
    .await;
    assert_eq!(second.status(), 200);
    let second = to_bytes(second.into_body(), usize::MAX).await.unwrap();
    let second: Value = serde_json::from_slice(&second).unwrap();
    assert_eq!(second["conversationId"], conversation_id);
    assert_eq!(second["result"], "Profile complete.");
    let conversation = fs::read_to_string(
        directory
            .join("history")
            .join(&flow_id)
            .join("conversations")
            .join(format!("{conversation_id}.json")),
    )
    .unwrap();
    let conversation: Value = serde_json::from_str(&conversation).unwrap();
    assert_eq!(conversation["status"], "completed");
    assert_eq!(conversation["messages"].as_array().unwrap().len(), 4);
    assert_eq!(
        conversation["messages"][1]["content"],
        "ASK: What is your name?"
    );
    assert_eq!(conversation["messages"][2]["content"], "Ana");
    fs::remove_dir_all(directory).unwrap();
}
