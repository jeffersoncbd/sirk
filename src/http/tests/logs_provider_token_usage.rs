use super::{flow::flow, request::request};
use axum::body::to_bytes;
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    time::{SystemTime, UNIX_EPOCH},
};

#[tokio::test]
async fn logs_provider_token_usage_without_returning_it_to_the_client() {
    let directory = std::env::temp_dir().join(format!(
        "sirk-token-usage-test-{}-{}",
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
        "#!/bin/sh\nprintf '%s' '{\"choices\":[{\"message\":{\"content\":\"Done.\"}}],\"usage\":{\"prompt_tokens\":12,\"completion_tokens\":4}}'\n",
    )
    .unwrap();
    let mut permissions = fs::metadata(&adapter).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&adapter, permissions).unwrap();
    fs::write(directory.join(".env"), "OPENROUTER_API_KEY=test-key\n").unwrap();
    fs::write(
        directory.join(".agents/token-agent.md"),
        format!(
            "---\nadapter: openrouter\nmodel: openai/gpt-5\ncall_prefix: '{}'\n---\nReply concisely.",
            adapter.display()
        ),
    )
    .unwrap();
    let flow_id = flow(&directory).await;
    let response = request(
        "POST",
        "/v1/agent/run",
        serde_json::json!({
            "directory": directory,
            "agent": "token-agent",
            "input": "Reply"
        })
        .to_string(),
        Some(&flow_id),
    )
    .await;
    assert_eq!(response.status(), 200);
    let response = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let response: serde_json::Value = serde_json::from_slice(&response).unwrap();
    assert_eq!(response["result"], "Done.");
    assert!(response.get("conversationId").is_none());
    let transcript =
        fs::read_to_string(directory.join("history").join(&flow_id).join("flow.log")).unwrap();
    assert!(
        transcript.contains("==> USAGE\nadapter: openrouter\ninput_tokens: 12\noutput_tokens: 4")
    );
    let response = request(
        "POST",
        "/v1/agent/run",
        serde_json::json!({
            "directory": directory,
            "agent": "token-agent",
            "input": "Reply again"
        })
        .to_string(),
        Some(&flow_id),
    )
    .await;
    assert_eq!(response.status(), 200);
    let usage =
        fs::read_to_string(directory.join("history").join(&flow_id).join("usage.log")).unwrap();
    assert_eq!(
        usage,
        "openrouter: 2 calls - total_input_tokens: 24 - total_output_tokens: 8\n   - token-agent (openai/gpt-5)\n   - token-agent (openai/gpt-5)\n"
    );
    fs::remove_dir_all(directory).unwrap();
}
