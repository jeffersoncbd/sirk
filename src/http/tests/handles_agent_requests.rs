use super::{flow::flow, request::request};
use axum::body::to_bytes;
use serde_json::Value;
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    time::{SystemTime, UNIX_EPOCH},
};

#[tokio::test]
async fn handles_agent_requests() {
    let directory = std::env::temp_dir().join(format!(
        "sirk-http-test-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(directory.join(".agents")).unwrap();
    let adapter = directory.join("adapter");
    fs::write(&adapter, "#!/bin/sh\nprintf '%s' 'served over HTTP'\n").unwrap();
    let mut permissions = fs::metadata(&adapter).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&adapter, permissions).unwrap();
    fs::write(
        directory.join(".agents/code-explainer.md"),
        format!(
            "---\nadapter: codex\ncall_prefix: '{}'\n---\nExplain code.",
            adapter.display()
        ),
    )
    .unwrap();
    let body = serde_json::json!({
        "directory": directory,
        "agent": "code-explainer",
        "input": "Explain this file"
    })
    .to_string();
    let flow_id = flow(&directory).await;
    let response = request("POST", "/v1/agent/run", body, Some(&flow_id)).await;
    assert_eq!(response.status(), 200);
    let response = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let response: Value = serde_json::from_slice(&response).unwrap();
    assert_eq!(response["result"], "served over HTTP", "{response}");
    assert!(response.get("ask").is_none(), "{response}");
    assert!(response.get("conversationId").is_none(), "{response}");
    let transcript =
        fs::read_to_string(directory.join("history").join(&flow_id).join("flow.log")).unwrap();
    assert!(transcript.contains("==> INPUT\nExplain code."));
    assert!(transcript.contains("<== OUTPUT\nserved over HTTP"));
    assert!(
        fs::read_dir(
            directory
                .join("history")
                .join(&flow_id)
                .join("conversations")
        )
        .unwrap()
        .next()
        .is_none()
    );
    fs::remove_dir_all(directory).unwrap();
}
