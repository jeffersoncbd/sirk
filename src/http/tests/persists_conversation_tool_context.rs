use super::{flow::flow, request::request};
use axum::body::to_bytes;
use serde_json::Value;
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    time::{SystemTime, UNIX_EPOCH},
};

#[tokio::test]
async fn persists_tool_context_between_conversation_requests() {
    let directory = std::env::temp_dir().join(format!(
        "sirk-http-conversation-tool-test-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(directory.join(".agents")).unwrap();
    fs::write(directory.join("fact.txt"), "stored fact").unwrap();
    let adapter = directory.join("adapter");
    fs::write(
        &adapter,
        "#!/bin/sh\ncase \"$*\" in\n  *'ASK: Continue?'*'User:'*'yes'*) printf '%s' 'Used stored fact.' ;;\n  *'Tool result (READ):'*'stored fact'*) printf '%s' 'ASK: Continue?' ;;\n  *) printf '%s' 'READ: fact.txt' ;;\nesac\n",
    )
    .unwrap();
    let mut permissions = fs::metadata(&adapter).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&adapter, permissions).unwrap();
    fs::write(
        directory.join(".agents/reader.md"),
        format!(
            "---\nadapter: codex\ncall_prefix: '{}'\nASK_TOOL: allow\n---\nRead the fact, ask before using it, and then finish.",
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
            "agent": "reader",
            "input": "Start."
        })
        .to_string(),
        Some(&flow_id),
    )
    .await;
    assert_eq!(first.status(), 200);
    let first = to_bytes(first.into_body(), usize::MAX).await.unwrap();
    let first: Value = serde_json::from_slice(&first).unwrap();
    assert_eq!(first["ask"], "Continue?");
    let conversation_id = first["conversationId"].as_str().unwrap();
    let second = request(
        "POST",
        "/v1/agent/run",
        serde_json::json!({
            "directory": directory,
            "agent": "reader",
            "input": "yes",
            "conversationId": conversation_id
        })
        .to_string(),
        Some(&flow_id),
    )
    .await;
    assert_eq!(second.status(), 200);
    let second = to_bytes(second.into_body(), usize::MAX).await.unwrap();
    let second: Value = serde_json::from_slice(&second).unwrap();
    assert_eq!(second["result"], "Used stored fact.");
    let conversation = fs::read_to_string(
        directory
            .join("history")
            .join(&flow_id)
            .join("conversations")
            .join(format!("{conversation_id}.json")),
    )
    .unwrap();
    let conversation: Value = serde_json::from_str(&conversation).unwrap();
    assert_eq!(conversation["messages"][2]["role"], "tool");
    assert_eq!(conversation["messages"][2]["name"], "READ");
    assert_eq!(conversation["messages"][2]["content"], "stored fact");
    fs::remove_dir_all(directory).unwrap();
}
