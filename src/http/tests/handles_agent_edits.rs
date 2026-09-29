use super::request::request;
use axum::body::to_bytes;
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    time::{SystemTime, UNIX_EPOCH},
};

#[tokio::test]
async fn handles_agent_edits() {
    let directory = std::env::temp_dir().join(format!(
        "sirk-agent-edit-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(directory.join(".agents")).unwrap();
    fs::write(directory.join("note.txt"), "before").unwrap();
    let adapter = directory.join("adapter");
    fs::write(&adapter, "#!/bin/sh\ncase \"$*\" in\n  *'Tool result (EDIT):'*) printf '%s' 'edited' ;;\n  *) printf '%s' 'EDIT: {\"path\":\"note.txt\",\"operation\":\"append\",\"input\":\" after\"}' ;;\nesac\n").unwrap();
    let mut permissions = fs::metadata(&adapter).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&adapter, permissions).unwrap();
    fs::write(
        directory.join(".agents/editor.md"),
        format!(
            "---\nadapter: codex\ncall_prefix: '{}'\nEDIT_TOOL: allow\n---\nEdit the note.",
            adapter.display()
        ),
    )
    .unwrap();
    let body = serde_json::json!({
        "directory": directory,
        "agent": "editor",
        "input": "Append text to note.txt"
    })
    .to_string();
    let response = request("POST", "/v1/agent/run", body).await;
    let status = response.status();
    let response = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(status, 200, "{}", String::from_utf8_lossy(&response));
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&response).unwrap()["result"],
        "edited"
    );
    assert_eq!(
        fs::read_to_string(directory.join("note.txt")).unwrap(),
        "before after"
    );
    fs::remove_dir_all(directory).unwrap();
}
