use super::super::handle::handle;
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn handles_agent_edits() {
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
    let response = handle("POST", "/v1/agent/run", &body);
    assert_eq!(response.status, 200, "{}", response.body);
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&response.body).unwrap()["result"],
        "edited"
    );
    assert_eq!(
        fs::read_to_string(directory.join("note.txt")).unwrap(),
        "before after"
    );
    fs::remove_dir_all(directory).unwrap();
}
