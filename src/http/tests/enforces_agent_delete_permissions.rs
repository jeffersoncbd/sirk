use super::request::request;
use axum::body::to_bytes;
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    time::{SystemTime, UNIX_EPOCH},
};

#[tokio::test]
async fn enforces_agent_delete_permissions() {
    let directory = std::env::temp_dir().join(format!(
        "sirk-delete-permissions-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(directory.join(".agents")).unwrap();
    fs::write(directory.join("note.txt"), "keep until allowed").unwrap();
    let adapter = directory.join("adapter");
    fs::write(
        &adapter,
        "#!/bin/sh\ncase \"$*\" in\n  *'Tool result (DELETE):'*) printf '%s' 'done' ;;\n  *) printf '%s' 'DELETE: {\"path\":\"note.txt\",\"force\":true}' ;;\nesac\n",
    )
    .unwrap();
    let mut permissions = fs::metadata(&adapter).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&adapter, permissions).unwrap();
    for (name, metadata, should_delete) in [
        ("restricted", "DELETE_TOOL: allow", false),
        (
            "allowed",
            "DELETE_TOOL: allow\nDELETE_WITHOUT_CONFIRM: allow",
            true,
        ),
    ] {
        fs::write(
            directory.join(format!(".agents/{name}.md")),
            format!(
                "---\nadapter: codex\ncall_prefix: '{}'\n{metadata}\n---\nHandle the file.",
                adapter.display()
            ),
        )
        .unwrap();
        let body = serde_json::json!({
            "directory": directory,
            "agent": name,
            "input": "Delete note.txt"
        })
        .to_string();
        let response = request("POST", "/v1/agent/run", body).await;
        let status = response.status();
        let response = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(status, 200, "{}", String::from_utf8_lossy(&response));
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&response).unwrap()["result"],
            "done"
        );
        assert_eq!(!directory.join("note.txt").exists(), should_delete);
    }
    fs::remove_dir_all(directory).unwrap();
}
