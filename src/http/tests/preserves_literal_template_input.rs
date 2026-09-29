use super::{flow::flow, request::request};
use axum::body::to_bytes;
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    time::{SystemTime, UNIX_EPOCH},
};

#[tokio::test]
async fn preserves_literal_template_input() {
    let directory = std::env::temp_dir().join(format!(
        "sirk-literal-input-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(directory.join(".agents")).unwrap();
    let adapter = directory.join("adapter");
    fs::write(&adapter, "#!/bin/sh\ncase \"$*\" in\n  *'{{ outputs.plan }}'*) printf '%s' 'literal preserved' ;;\n  *) exit 4 ;;\nesac\n").unwrap();
    let mut permissions = fs::metadata(&adapter).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&adapter, permissions).unwrap();
    fs::write(
        directory.join(".agents/reviewer.md"),
        format!(
            "---\nadapter: codex\ncall_prefix: '{}'\n---\nReview the input.",
            adapter.display()
        ),
    )
    .unwrap();
    let body = serde_json::json!({
        "directory": directory,
        "agent": "reviewer",
        "input": "File content: {{ outputs.plan }}"
    })
    .to_string();
    let flow_id = flow(&directory).await;
    let response = request("POST", "/v1/agent/run", body, Some(&flow_id)).await;
    let status = response.status();
    let response = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(status, 200, "{}", String::from_utf8_lossy(&response));
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&response).unwrap()["result"],
        "literal preserved"
    );
    fs::remove_dir_all(directory).unwrap();
}
