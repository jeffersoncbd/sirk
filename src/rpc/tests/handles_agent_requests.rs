use super::super::{handle::handle, types::Request};
use serde_json::json;
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn handles_agent_requests() {
    let directory = std::env::temp_dir().join(format!(
        "sirk-rpc-test-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(directory.join(".agents")).unwrap();
    fs::write(directory.join("source.txt"), "content from the project").unwrap();
    let adapter = directory.join("adapter");
    fs::write(
        &adapter,
        "#!/bin/sh\ncase \"$*\" in\n  *'Tool result (READ)'*) printf '%s' 'documented' ;;\n  *) printf '%s' 'READ: source.txt' ;;\nesac\n",
    )
    .unwrap();
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
    let response = handle(
        Request {
            jsonrpc: "2.0".to_owned(),
            id: Some(json!(9)),
            method: "agent.run".to_owned(),
            params: json!({
                "agent": "code-explainer",
                "input": "Explain this file"
            }),
        },
        &directory,
    );
    assert_eq!(response.result.as_deref(), Some("documented"));
    assert!(response.error.is_none());
    fs::remove_dir_all(directory).unwrap();
}
